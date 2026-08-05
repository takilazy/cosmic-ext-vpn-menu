app-title = Trình đơn VPN COSMIC

# Popup states
no-vpn-connections = Không có kết nối VPN
load-error = Không thể tải các kết nối VPN: { $error }

# Connection status
vpn-status-connected = Đã kết nối
vpn-status-connecting = Đang kết nối…
vpn-status-disconnecting = Đang ngắt kết nối…
vpn-status-disconnected = Đã ngắt kết nối
vpn-status-unknown = Không rõ

# Connection detail panel
detail-type = Loại
detail-gateway = Cổng
detail-ipv4 = IPv4
detail-ipv6 = IPv6
detail-duration = Đã kết nối trong
autoconnect = Tự động kết nối

# Forget / delete
forget = Quên
confirm-forget = Quên kết nối này?
delete = Xóa
cancel = Hủy

# Secret prompt
secret-prompt = Nhập thông tin xác thực cho { $name }
connect = Kết nối

# Import
import = Nhập…

# Create / edit
add-vpn = Thêm VPN
edit = Chỉnh sửa…
select-vpn-type = Chọn loại VPN
new-vpn = VPN { $kind } mới
edit-vpn = Chỉnh sửa VPN { $kind }
name-placeholder = Tên
advanced-data = Nâng cao (dữ liệu)
key-placeholder = Khóa
value-placeholder = Giá trị
add-field = Thêm trường
save = Lưu

# WireGuard editor
wg-private-key = Khóa riêng tư
wg-address = Địa chỉ
wg-peer = Đối tác
wg-peer-key = Khóa công khai
wg-peer-endpoint = Điểm cuối
wg-allowed-ips = IP được phép

# WireGuard / secrets (added)
dns-servers = Máy chủ DNS
wg-mtu = MTU
wg-peer-n = Đối tác { $index }
wg-preshared-key = Khóa chia sẻ trước
wg-keepalive = Giữ kết nối liên tục (giây)
wg-add-peer = Thêm đối tác
remove = Gỡ bỏ
secret-store = Lưu mật khẩu
secret-ask = Luôn hỏi
secret-none = Không bắt buộc

# Sync (backfilled)
notify-connected = Đã kết nối VPN
notify-disconnected = Đã ngắt kết nối VPN
notify-failed = Không thể kết nối VPN
reason-ip-config = cấu hình IP không hợp lệ
reason-timeout = kết nối đã hết thời gian chờ
reason-service-failed = dịch vụ VPN không khởi động được
reason-no-secrets = chưa cung cấp mật khẩu
reason-login-failed = xác thực thất bại
detail-dns = DNS
detail-received = Đã nhận
detail-sent = Đã gửi
detail-last-used = Lần dùng gần nhất
duplicate = Nhân bản
priority = Mức ưu tiên tự động kết nối
metered = Có tính lưu lượng
metered-auto = Tự động
metered-yes = Có
metered-no = Không
zone = Vùng tường lửa
split-tunnel = Chỉ dùng cho mạng của riêng nó
dns-search = Miền tìm kiếm DNS

# OpenConnect / OpenVPN / vpnc (added)
oc-general = Chung
oc-cert-auth = Xác thực bằng chứng chỉ
oc-protocol = Giao thức VPN
oc-ca-cert = Chứng chỉ CA
oc-proxy = Proxy
oc-user-agent = User Agent
oc-reported-version = Phiên bản báo cáo
oc-reported-os = Hệ điều hành báo cáo
oc-os-default = Mặc định
oc-csd-trojan = Cho phép trojan Cisco Secure Desktop
oc-csd-wrapper = Tập lệnh bao bọc CSD
oc-machine-cert = Chứng chỉ máy
oc-private-key = Khóa riêng tư
oc-user-cert = Chứng chỉ người dùng
oc-fsid = Dùng FSID cho cụm mật khẩu của khóa
oc-prevent-invalid = Ngăn người dùng tự chấp nhận các chứng chỉ không hợp lệ
ovpn-conn-type = Loại kết nối
ovpn-ct-tls = Chứng chỉ (TLS)
ovpn-ct-password = Mật khẩu
ovpn-ct-password-tls = Mật khẩu kèm chứng chỉ (TLS)
ovpn-ct-static-key = Khóa tĩnh
ovpn-username = Tên người dùng
ovpn-static-key = Khóa tĩnh
ovpn-key-dir = Hướng khóa
ovpn-remote-ip = Địa chỉ IP từ xa
ovpn-local-ip = Địa chỉ IP cục bộ
ovpn-advanced = Nâng cao
ovpn-port = Cổng gateway
ovpn-tcp = Dùng kết nối TCP
ovpn-cipher = Mật mã
ovpn-auth = Xác thực HMAC
vpnc-group = Tên nhóm
vpnc-domain = Miền
vpnc-nat = Xuyên NAT
vpnc-pfs = Perfect Forward Secrecy
vpnc-dh = Nhóm DH
vpnc-vendor = Nhà cung cấp
vpnc-app-version = Phiên bản ứng dụng
vpnc-single-des = Bật mã hóa single DES yếu
