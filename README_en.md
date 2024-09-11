# rchat

`rchat` is a chat application designed to provide simple and efficient chatting capabilities.

# Chat System - Customer Support System
[中文](README.md)
## ✨ Features
- 🌹 Embed in websites as a chat support system
- 🌹 Initiate chats via direct links
- 🌹 Backend conversation management

## ✨ Highlights
- 🐍 Message notifications
- 🐍 Low resource consumption
- 🐍 Ready to use out of the box
- 🐍 Minimal configuration
- 🐍 Developed in Rust

## ✨ Example
**TODO:**
Record a demonstration video

## Startup Instructions

1. Clone the repository:

    ```bash
    git clone https://github.com/AdairStone/rchat.git
    ```

2. Enter the project directory and start the service:

    ```bash
    cd rchat
    docker-compose up -d
    ```

3. After modifying the configuration, restart the service:

    ```bash
    docker restart rchat
    ```

4. Visit [http://hostname:18889](http://hostname:18889) to configure the website and manage conversations.

    - **Username**: admin
    - **Password**: Aa123456

5. After successfully configuring the website, verify the service:

    [http://hostname:18888/load/direct?key=pbAuq7PVr2gh2jp&rand=128](http://hostname:18888/load/direct?key=pbAuq7PVr2gh2jp&rand=128)

## License

This project is licensed under the MIT License. For more details, please refer to the [LICENSE.txt](LICENSE.txt) file.

## Contact

If you have any questions or suggestions, please contact us via the [issue tracker](https://github.com/AdairStone/rchat/issues).

## 联系我：备注（rchat）
![wechat of ada](wechat.png)