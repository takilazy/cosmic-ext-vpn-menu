app-title = COSMIC VPN 菜单

# Popup states
no-vpn-connections = 无 VPN 连接
load-error = 无法加载 VPN 连接：{ $error }

# Connection status
vpn-status-connected = 已连接
vpn-status-connecting = 正在连接…
vpn-status-disconnecting = 正在断开…
vpn-status-disconnected = 已断开
vpn-status-unknown = 未知

# Connection detail panel
detail-type = 类型
detail-gateway = 网关
detail-ipv4 = IPv4
detail-ipv6 = IPv6
detail-duration = 已连接时长
autoconnect = 自动连接

# Forget / delete
forget = 忘记
confirm-forget = 忘记此连接？
delete = 删除
cancel = 取消

# Secret prompt
secret-prompt = 输入 { $name } 的凭据
connect = 连接

# Import
import = 导入…

# Create / edit
add-vpn = 添加 VPN
edit = 编辑…
select-vpn-type = 选择 VPN 类型
new-vpn = 新建 { $kind } VPN
edit-vpn = 编辑 { $kind } VPN
name-placeholder = 名称
advanced-data = 高级（数据）
key-placeholder = 键
value-placeholder = 值
add-field = 添加字段
save = 保存

# WireGuard editor
wg-private-key = 私钥
wg-address = 地址
wg-peer = 对等端
wg-peer-key = 公钥
wg-peer-endpoint = 端点
wg-allowed-ips = 允许的 IP

# WireGuard / secrets (added)
dns-servers = DNS 服务器
wg-mtu = MTU
wg-peer-n = 对等端 { $index }
wg-preshared-key = 预共享密钥
wg-keepalive = 持久保活（秒）
wg-add-peer = 添加对等端
remove = 移除
secret-store = 保存密码
secret-ask = 始终询问
secret-none = 不需要

# Sync (backfilled)
notify-connected = VPN 已连接
notify-disconnected = VPN 已断开
notify-failed = VPN 连接失败
reason-ip-config = IP 配置无效
reason-timeout = 连接超时
reason-service-failed = VPN 服务启动失败
reason-no-secrets = 未提供密码
reason-login-failed = 身份验证失败
detail-dns = DNS
detail-received = 已接收
detail-sent = 已发送
detail-last-used = 上次使用
duplicate = 复制
priority = 自动连接优先级
metered = 计费连接
metered-auto = 自动
metered-yes = 是
metered-no = 否
zone = 防火墙区域
split-tunnel = 仅用于其自身网络
dns-search = DNS 搜索域

# OpenConnect / OpenVPN / vpnc (added)
oc-general = 常规
oc-cert-auth = 证书认证
oc-protocol = VPN 协议
oc-ca-cert = CA 证书
oc-proxy = 代理
oc-user-agent = User Agent
oc-reported-version = 报告的版本
oc-reported-os = 报告的操作系统
oc-os-default = 默认
oc-csd-trojan = 允许 Cisco Secure Desktop 木马
oc-csd-wrapper = CSD 包装脚本
oc-machine-cert = 机器证书
oc-private-key = 私钥
oc-user-cert = 用户证书
oc-fsid = 使用 FSID 作为密钥口令
oc-prevent-invalid = 阻止用户手动接受无效证书
ovpn-conn-type = 连接类型
ovpn-ct-tls = 证书（TLS）
ovpn-ct-password = 密码
ovpn-ct-password-tls = 密码加证书（TLS）
ovpn-ct-static-key = 静态密钥
ovpn-username = 用户名
ovpn-static-key = 静态密钥
ovpn-key-dir = 密钥方向
ovpn-remote-ip = 远程 IP 地址
ovpn-local-ip = 本地 IP 地址
ovpn-advanced = 高级
ovpn-port = 网关端口
ovpn-tcp = 使用 TCP 连接
ovpn-cipher = 加密算法
ovpn-auth = HMAC 认证
vpnc-group = 组名
vpnc-domain = 域
vpnc-nat = NAT 穿透
vpnc-pfs = 完全前向保密
vpnc-dh = DH 组
vpnc-vendor = 厂商
vpnc-app-version = 应用程序版本
vpnc-single-des = 启用弱 single DES 加密
