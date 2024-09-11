# rchat

`rchat` 是一个聊天应用程序，旨在提供简单且高效的聊天功能。

# 聊天系统 - 客服系统
[English](README_en.md)
## ✨ 功能
- 🌹 嵌入网站作为聊天客服
- 🌹 通过链接直接发起聊天
- 🌹 后台会话管理

## ✨ 特色
- 🐍 消息提醒
- 🐍 资源占用少
- 🐍 开箱即用
- 🐍 配置少
- 🐍 Rust开发

## ✨ 示例
**TODO：**
录制一个功能演示视频

## 启动步骤

1. 克隆仓库：

    ```bash
    git clone https://github.com/AdairStone/rchat.git
    ```

2. 进入项目目录并启动服务：

    ```bash
    cd rchat
    docker-compose up -d
    ```

3. 修改配置后，重启服务：

    ```bash
    docker restart rchat
    ```

4. 访问 [http://hostname:18889](http://hostname:18889) 配置网站并进行会话管理

    - **用户名**：admin
    - **密码**：Aa123456

5. 配置网站成功后，验证服务：

    [http://hostname:18888/load/direct?key=pbAuq7PVr2gh2jp&rand=128](http://hostname:18888/load/direct?key=pbAuq7PVr2gh2jp&rand=128)

## 许可证

该项目采用 MIT 许可证。有关详细信息，请参阅 [LICENSE.txt](LICENSE.txt) 文件。

## 联系

如果你有任何问题或建议，请通过 [issue tracker](https://github.com/AdairStone/rchat/issues) 联系我们。

## 联系我：备注（rchat）
![wechat of ada](wechat.png)