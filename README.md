# AnyMaku

**Danmaku Anywhere**

## 项目简介

AnyMaku 是一个旨在实现“随处弹幕（Danmaku Anywhere）”的桌面端应用。本项目采用现代化的 Tauri 技术栈，为用户提供流畅的桌面弹幕体验。可以说是哔哩哔哩弹幕姬的下一代版本。

技术栈
- 后端/桌面框架: Tauri v2
- 前端框架: Vue 3
- 编程语言: TypeScript 与 Rust
- 构建工具: Vite 与 Node.js

## 项目截图

![控制台](https://github.com/arkerny/AnyMaku/blob/main/imgs/Snipaste_2026-01-31_23-03-51.png?raw=true)

![效果 1](https://github.com/arkerny/AnyMaku/blob/main/imgs/Snipaste_2026-01-31_23-05-33.png?raw=true)

![效果 2](https://github.com/arkerny/AnyMaku/blob/main/imgs/Snipaste_2026-01-31_23-06-15.png?raw=true)

## 项目结构

项目主要代码分布如下：
- src-tauri/: Rust 源代码及 Tauri 应用配置，负责系统原生功能集成。
- src/: 前端 Vue 3 源代码，包含组件、逻辑与样式。
- public/: 静态资源文件。
- package.json: Node.js 依赖管理及运行脚本。
- vite.config.ts: 前端构建配置。

## 开发指引

如果您希望参与项目的开发或在本地运行，请参考以下步骤：

1. 环境准备

在开始之前，请确保您的开发环境已安装以下工具：
- Node.js: 用于运行前端构建环境。
- Rust: Tauri 的核心编译环境（建议通过 rustup 安装）。
- Tauri CLI: 可通过 npm install -g @tauri-apps/cli 安装。

2. 获取代码

``` sh
git clone https://github.com/arkerny/AnyMaku.git
cd AnyMaku
```

3. 安装依赖

使用 Node.js 包管理器安装前端和 Tauri 相关的依赖：

``` sh
npm install
```

4. 启动开发服务器
运行以下命令启动包含热更新的开发环境：

``` sh
npm run tauri dev
```

## 参与贡献

发起 Pull Request。

## 开源协议

本项目采用 GPL-3.0 license 开源协议。