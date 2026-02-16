# ncmdownloader

用于下载网易云歌单中音乐的脚本

# 下载

Github Release

# 构建

运行`cargo build --release`安装依赖并构建

# 登录方式

手机号登录，并且可以将登录信息保存在本地

# 支持功能

- 为音乐添加封面、标题、作者、专辑
- 下载歌词
- 并发下载

# 使用方法

- Windows: 运行`ncmdownloader.exe`，登录后输入要下载的歌单Id即可下载
- Linux/MacOS: 运行`ncmdownloader`，登录后输入要下载的歌单Id即可下载

# 配置文件

修改`config.yml`

- max_bitrate_level: 下载歌曲的最高质量，不填写内容默认为可下载的最高质量

    可填内容:
    - higher: 较高
    - exhigh: 极高
    - lossless: 无损
    - hires: Hi-Res
    - jyeffect: 高清环绕声
    - sky: 沉浸环绕声
    - dolby: 杜比全景声
    - jymaster: 超清母带

- download_songs: 是否下载歌曲

    可填内容:
    - true: 下载歌曲
    - false: 不下载歌曲

- download_lyrics: 是否下载歌词

    可填内容:
    - true: 下载歌词
    - false: 不下载歌词

- concurrency: 同时下载的任务数

    可填内容: 正整数(不建议设置太大)

- retry: 下载失败时的重试次数

    可填内容: 正整数

- retry_delay: 下载失败时的重试间隔时间

    可填内容: 正整数(单位：毫秒)

- timeout: 下载超时时间

    可填内容: 正整数(单位：毫秒)
