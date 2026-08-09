# herdr-cn · herdr 简体中文版

> **非官方汉化发行版** · 基于 [herdr](https://github.com/herdrdev/herdr) v0.8.0 · Apache-2.0
>
> 本项目与 herdr 官方无隶属关系。"herdr" 名称属于其原作者，此处仅作描述性使用。

herdr 是一个为 AI 编码 agent（Claude Code、Codex、Gemini CLI 等）设计的终端多路复用器：多工作区、多窗格并行管理 agent 会话，自动检测 agent 状态（运行中 / 待响应 / 完成），鼠标优先操作。本仓库提供**全界面简体中文**的编译发行版。

## 与官方版的差异

- **全界面汉化**：侧边栏、菜单、对话框、设置、快捷键帮助、导航器、通知等全部用户可见文案（协议字段、配置键、CLI 保持原样以兼容生态）
- **CJK 显示修复**：按钮 / 菜单宽度按 Unicode 显示宽度计算，中文不截断、点击区域不偏移
- **更新渠道指向 herdr-cn**：`herdr update` 与内置的「更新就绪」提示获取的都是本仓库发布的新版汉化包（SHA-256 校验照常），不会被上游英文版覆盖；preview 通道暂不提供
- **Windows 支持 stable 通道**：上游的 Windows 版只能走 preview 通道，herdr-cn 全平台统一 stable

除以上改动外，功能与官方 v0.8.0 完全一致。官方原版说明见 [README.upstream.md](README.upstream.md)（英文）与 [README.zh-CN.md](README.zh-CN.md)（官方中文简介）。

## 安装

**macOS**（Apple Silicon 用 `herdr-macos-aarch64`，Intel 用 `herdr-macos-x86_64`）：

```bash
curl -L -o herdr https://github.com/Win-Hao/herdr-cn/releases/latest/download/herdr-macos-aarch64
chmod +x herdr
xattr -d com.apple.quarantine herdr 2>/dev/null || true
sudo mv herdr /usr/local/bin/herdr
```

**Linux**（x86_64 / aarch64，musl 静态链接，任何发行版可用）：

```bash
curl -L -o herdr https://github.com/Win-Hao/herdr-cn/releases/latest/download/herdr-linux-x86_64
chmod +x herdr && sudo mv herdr /usr/local/bin/herdr
```

**Windows**（PowerShell，自动下载安装并加入 PATH）：

```powershell
irm https://raw.githubusercontent.com/Win-Hao/herdr-cn/zh-cn/website/install.ps1 | iex
```

若之前在用官方英文版，先执行 `herdr server stop` 停掉旧服务再启动中文版（工作区布局会自动恢复）。安装后 `herdr update` 即可获取后续汉化版更新。

## 从源码编译

需要 Rust（rust-toolchain.toml 指定版本，rustup 自动处理）和 **zig 0.15.2**（`brew install zig@0.15`）：

```bash
export PATH="$(brew --prefix zig@0.15)/bin:$PATH"
cargo build --release
```

大陆网络下 zig 拉取依赖若报 `TlsInitializationFailed`，用 curl 手动下载 `vendor/libghostty-vt` 下各 `build.zig.zon` 中的依赖包后逐个 `zig fetch <文件>` 即可。

## 跟进上游

本仓库跟随上游 stable release 更新：上游发版 → 同步汉化 → 测试（与上游基线零回归）→ 发布对应的 `-cn` 版本。

## 许可证与致谢

- 上游 herdr 由 [herdrdev](https://github.com/herdrdev/herdr) 开发，Apache License 2.0，原 [LICENSE](LICENSE) 完整保留
- 本仓库的修改（汉化及适配，见提交历史）同样以 Apache License 2.0 发布
- 感谢上游对中文社区的友好（官方 README 提供中文版、已合并多个中文文档 PR）
