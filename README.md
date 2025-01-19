## 自建 tailscale 中转服务器（derp）

这是一个简易的 derper 客户端验证程序，只有白名单内的客户端才允许使用。

### 使用方法

1. 下载 https://github.com/wzv5/derp-client-verifier/raw/refs/heads/main/docker-compose.yml
2. 修改 docker-compose.yml 文件中的 hostname 参数为自己的域名
3. 以本仓库 data 目录为模板，创建自己的 data 目录，在其中放入 client_list.txt 和 证书文件
4. docker compose up
5. 注意，为了便于设置，默认 stun port 也为 11443，防火墙需要放行 11443 端口的 tcp 和 udp

### 客户端列表文件（client_list.txt）格式说明

- 从 tailscale 后台复制设备的 Node key，粘贴到 client_list.txt 文件中，一行一个
- 如果一行以 # 开头，则为注释，注释必须为单独的一行，不支持行末注释

### 感谢

- https://github.com/fredliang44/derper-docker
