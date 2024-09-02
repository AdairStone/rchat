# rchat

`rchat` 是一个聊天应用程序，旨在提供简单且高效的聊天功能。该项目包含一个 Rust 后端和一个 Vue 3 前端。

## 特性

- 实时聊天功能
- 支持文件上传和预览
- 表情符号选择
- WebSocket 通信

## 目录结构

rchat/ │ ├── backend/ │ ├── src/ │ ├── Cargo.toml │ └── ... │ ├── frontend/ │ ├── src/ │ ├── package.json │ └── ... │ └── README.md

## 快速开始

### 后端

1. 克隆仓库：

    ```bash
    git clone https://github.com/AdairStone/rchat.git
    cd rchat/backend
    ```

2. 安装依赖并构建：

    ```bash
    cargo build
    ```

3. 运行后端服务器：

    ```bash
    cargo run
    ```

### 前端

1. 克隆仓库并切换到前端目录：

    ```bash
    cd rchat/frontend
    ```

2. 安装依赖：

    ```bash
    npm install
    ```

3. 运行前端开发服务器：

    ```bash
    npm run dev
    ```

4. 打开浏览器并访问 `http://localhost:3000` 以查看应用程序。

## 配置

### 后端配置

- 配置文件位于 `backend/config/` 目录下。
- 修改 `config.toml` 以调整服务器设置，如端口号和数据库配置。

### 前端配置

- 配置文件位于 `frontend/src/config/` 目录下。
- 修改 `config.ts` 以调整 API 端点和其他前端设置。

## 贡献

欢迎贡献代码！请遵循以下步骤：

1. Fork 本仓库并克隆到本地。
2. 创建一个新的分支进行你的更改。
3. 提交更改并推送到你的分支。
4. 创建一个 Pull Request。

## 许可证

该项目采用 MIT 许可证。有关详细信息，请参阅 [LICENSE](LICENSE) 文件。

## 联系

如果你有任何问题或建议，请通过 [issue tracker](https://github.com/AdairStone/rchat/issues) 联系我们。
