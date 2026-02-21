use anyhow::{Context, Result, bail};
use reqwest::{Client, ClientBuilder};
use std::path::Path;
use std::pin::Pin;
use std::task::{Context as TaskContext, Poll};
use std::time::Duration;
use tokio::io::{AsyncRead, ReadBuf};
use url::Url;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36";
const CHUNK_SIZE: usize = 8192;
pub struct DownloadOptions {
    /// Maximum number of retry attempts
    pub max_retries: usize,
    /// Initial retry delay (exponential backoff multiplier)
    pub retry_delay: Duration,
    /// Request timeout
    pub timeout: Duration,
}

impl DownloadOptions {
    pub fn new(max_retries: usize, retry_delay: Duration, timeout: Duration) -> Self {
        Self {
            max_retries,
            retry_delay,
            timeout,
        }
    }
}

impl Default for DownloadOptions {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay: Duration::from_secs(1),
            timeout: Duration::from_secs(60),
        }
    }
}

/// 异步下载文件到指定路径，支持流式写入、自动重试和超时
///
/// # Arguments
/// * `url` - 下载链接
/// * `output_path` - 输出文件路径
/// * `options` - 下载配置选项
///
/// # Returns
/// * `Result<u64>` - 下载的字节数
#[allow(unused)]
pub async fn download_file(url: &Url, output_path: &Path, options: DownloadOptions) -> Result<u64> {
    let client = ClientBuilder::new()
        .timeout(options.timeout)
        .user_agent(USER_AGENT)
        .build()
        .context("Failed to create HTTP client")?;

    let mut retries = 0usize;
    let mut current_delay = options.retry_delay;

    loop {
        match download_with_client(&client, url, output_path, CHUNK_SIZE).await {
            Ok(bytes) => return Ok(bytes),
            Err(e) if retries < options.max_retries => {
                retries += 1;
                log::warn!(
                    "Download failed (attempt {}/{}): {}, retrying in {:?}",
                    retries,
                    options.max_retries,
                    e,
                    current_delay
                );
                tokio::time::sleep(current_delay).await;
            }
            Err(e) => bail!(
                "Download failed after {} attempts: {}",
                options.max_retries,
                e
            ),
        }
    }
}

async fn download_with_client(
    client: &Client,
    url: &Url,
    output_path: &Path,
    chunk_size: usize,
) -> Result<u64> {
    let response = client
        .get(url.clone())
        .send()
        .await
        .with_context(|| format!("Failed to send request to {}", url))?;

    if !response.status().is_success() {
        bail!("HTTP error: {}", response.status());
    }

    let total_size = response
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok());

    let mut file = tokio::fs::File::create(output_path)
        .await
        .with_context(|| format!("Failed to create file at {}", output_path.display()))?;
    let mut downloaded: u64 = 0;

    // Convert reqwest byte stream to AsyncRead with configurable chunk size
    let stream = response.bytes_stream();
    let mut stream_reader = ByteStreamWrapper::new(Box::pin(stream));

    let mut buffer = vec![0u8; chunk_size];
    loop {
        let bytes_read = tokio::io::AsyncReadExt::read(&mut stream_reader, &mut buffer)
            .await
            .with_context(|| "Failed to read response body")?;

        if bytes_read == 0 {
            break;
        }

        tokio::io::AsyncWriteExt::write_all(&mut file, &buffer[..bytes_read])
            .await
            .with_context(|| "Failed to write to file")?;

        downloaded += bytes_read as u64;

        if let Some(total) = total_size {
            log::info!("Downloaded {}/{} bytes", downloaded, total);
        }
    }

    Ok(downloaded)
}

// Wrapper to convert reqwest byte stream to AsyncRead
struct ByteStreamWrapper<'a> {
    stream: Pin<
        Box<
            dyn futures_util::Stream<Item = Result<bytes::Bytes, reqwest::Error>>
                + Send
                + Unpin
                + 'a,
        >,
    >,
    buffer: bytes::BytesMut,
}

impl<'a> ByteStreamWrapper<'a> {
    fn new(
        stream: Pin<
            Box<
                dyn futures_util::Stream<Item = Result<bytes::Bytes, reqwest::Error>>
                    + Send
                    + Unpin
                    + 'a,
            >,
        >,
    ) -> Self {
        Self {
            stream,
            buffer: bytes::BytesMut::new(),
        }
    }
}

impl AsyncRead for ByteStreamWrapper<'_> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut TaskContext<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        if buf.remaining() == 0 {
            return Poll::Ready(Ok(()));
        }

        // Fill internal buffer if empty
        if self.buffer.is_empty() {
            match self.stream.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(chunk))) => {
                    self.buffer.extend_from_slice(&chunk);
                }
                Poll::Ready(Some(Err(e))) => {
                    return Poll::Ready(Err(std::io::Error::other(
                        format!("Stream error: {}", e),
                    )));
                }
                Poll::Ready(None) => {
                    // Stream ended
                    return Poll::Ready(Ok(()));
                }
                Poll::Pending => return Poll::Pending,
            }
        }

        // Copy data to user buffer
        let to_read = std::cmp::min(buf.remaining(), self.buffer.len());
        let data = self.buffer.split_to(to_read);
        buf.put_slice(&data);

        Poll::Ready(Ok(()))
    }
}
