<div align="center">
  <img src="img/icon.svg" alt="ncmdownloader Logo" width="120" height="120">
  <h1>🎵 ncmdownloader</h1>
  <p><b>极致轻量 · 跨平台 · 网易云音乐下载工具</b></p>

[![Release](https://img.shields.io/github/v/release/fecwaqw/ncmdownloader-rs?color=success&style=flat-square)](https://github.com/AouTzxc/Global-mouse/releases)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20MacOS%20%7C%20Linux-blue?style=flat-square)](###)
[![Build Status](https://img.shields.io/github/actions/workflow/status/fecwaqw/ncmdownloader-rs/release.yml?style=flat-square)](https://github.com/fecwaqw/ncmdownloader-rs/actions)

</div>

---

## ✨ 核心特性

- 🖼️ **元数据嵌入**：自动为下载的歌曲添加封面、标题、作者、专辑信息
- 📝 **歌词下载**：同步下载配套歌词（.lrc 格式）
- ⚡ **并发下载**：多任务同时进行，大幅提升批量下载效率
- 🔐 **登录态保存**：手机号登录后本地保存凭证，无需重复登录

---

## 🔑 登录方式

使用**手机号登录**，登录信息会加密保存在本地配置文件中，下次启动自动恢复会话。

---

## 🚀 使用方法

### 1. 下载对应版本

前往 [Releases 页面](https://github.com/fecwaqw/ncmdownloader-rs/releases) 下载最新版（Latest）适合你系统的文件：

| 操作系统              | 下载文件                                 |
| --------------------- | ---------------------------------------- |
| Windows               | `ncmdownloader-0.1.0-windows-x86_64.exe` |
| Linux                 | `ncmdownloader-0.1.0-linux-x86_64`       |
| macOS (Intel)         | `ncmdownloader-0.1.0-macos-x86_64`       |
| macOS (Apple Silicon) | `ncmdownloader-0.1.0-macos-arm64`        |

### 2. 首次运行

- **Windows**：双击 `.exe` 文件，程序会自动生成配置文件 `config.yml`，**编辑配置文件后再次运行即可登录**。
- **Linux / macOS**：给予执行权限（`chmod +x 文件名`），然后在终端中运行，同样会生成配置文件，编辑后重新运行。

---

## ⚙️ 配置文件详解

配置文件 `config.yml` 位于程序同目录下，所有选项及说明如下：

| 配置项              | 说明                                                 | 可填值                                                                                                                                                                        |
| ------------------- | ---------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `max_bitrate_level` | 下载歌曲的最高音质（留空则自动选择可获取的最高音质） | `higher` (较高)<br>`exhigh` (极高)<br>`lossless` (无损)<br>`hires` (Hi-Res)<br>`jyeffect` (高清环绕声)<br>`sky` (沉浸环绕声)<br>`dolby` (杜比全景声)<br>`jymaster` (超清母带) |
| `download_songs`    | 是否下载歌曲文件                                     | `true` / `false`                                                                                                                                                              |
| `download_lyrics`   | 是否下载歌词文件                                     | `true` / `false`                                                                                                                                                              |
| `concurrency`       | 同时下载的任务数（正整数，不宜过大）                 | 例如 `5`                                                                                                                                                                      |
| `retry`             | 下载失败重试次数（正整数）                           | 例如 `3`                                                                                                                                                                      |
| `retry_delay`       | 重试间隔时间（毫秒）                                 | 例如 `1000`                                                                                                                                                                   |
| `timeout`           | 下载超时时间（毫秒）                                 | 例如 `30000`                                                                                                                                                                  |

---

## 🛠️ 从源码构建

确保已安装 Rust 工具链，然后执行：

```bash
cargo build --release
```

编译后的可执行文件位于 `target/release/` 目录下。

---

## 🤝 贡献与反馈

- 发现 Bug？欢迎 [提交 Issue](https://github.com/fecwaqw/ncmdownloader-rs/issues)
- 有好的想法？欢迎 Fork 并提交 Pull Request

---

## 📄 许可证

本项目采用 **GPL-3.0** 许可证。  
您可以自由使用、修改和分发，但任何衍生产品**必须同样开源且采用 GPL 协议**。禁止闭源商业分发。

---

## ☕ 支持作者

如果这个工具对你有帮助，不妨请作者喝杯咖啡，鼓励持续维护和开发！

<p align="center">
  <img src="img/donate.jpg" width="250" alt="赞赏码" style="border-radius: 10px; box-shadow: 0 4px 8px rgba(0,0,0,0.2);">
</p>
