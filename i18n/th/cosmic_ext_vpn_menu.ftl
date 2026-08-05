app-title = เมนู VPN ของ COSMIC

# Popup states
no-vpn-connections = ไม่มีการเชื่อมต่อ VPN
load-error = ไม่สามารถโหลดการเชื่อมต่อ VPN: { $error }

# Connection status
vpn-status-connected = เชื่อมต่อแล้ว
vpn-status-connecting = กำลังเชื่อมต่อ…
vpn-status-disconnecting = กำลังยกเลิกการเชื่อมต่อ…
vpn-status-disconnected = ยกเลิกการเชื่อมต่อแล้ว
vpn-status-unknown = ไม่ทราบ

# Connection detail panel
detail-type = ประเภท
detail-gateway = เกตเวย์
detail-ipv4 = IPv4
detail-ipv6 = IPv6
detail-duration = เชื่อมต่อมาแล้ว
autoconnect = เชื่อมต่ออัตโนมัติ

# Forget / delete
forget = ลืม
confirm-forget = ลืมการเชื่อมต่อนี้หรือไม่?
delete = ลบ
cancel = ยกเลิก

# Secret prompt
secret-prompt = ป้อนข้อมูลรับรองสำหรับ { $name }
connect = เชื่อมต่อ

# Import
import = นำเข้า…

# Create / edit
add-vpn = เพิ่ม VPN
edit = แก้ไข…
select-vpn-type = เลือกประเภท VPN
new-vpn = VPN { $kind } ใหม่
edit-vpn = แก้ไข VPN { $kind }
name-placeholder = ชื่อ
advanced-data = ขั้นสูง (ข้อมูล)
key-placeholder = คีย์
value-placeholder = ค่า
add-field = เพิ่มฟิลด์
save = บันทึก

# WireGuard editor
wg-private-key = คีย์ส่วนตัว
wg-address = ที่อยู่
wg-peer = เพียร์
wg-peer-key = คีย์สาธารณะ
wg-peer-endpoint = ปลายทาง
wg-allowed-ips = IP ที่อนุญาต

# WireGuard / secrets (added)
dns-servers = เซิร์ฟเวอร์ DNS
wg-mtu = MTU
wg-peer-n = เพียร์ { $index }
wg-preshared-key = คีย์ที่แชร์ล่วงหน้า
wg-keepalive = คงการเชื่อมต่อถาวร (วินาที)
wg-add-peer = เพิ่มเพียร์
remove = ลบออก
secret-store = บันทึกรหัสผ่าน
secret-ask = ถามทุกครั้ง
secret-none = ไม่จำเป็น

# Sync (backfilled)
notify-connected = เชื่อมต่อ VPN แล้ว
notify-disconnected = ยกเลิกการเชื่อมต่อ VPN แล้ว
notify-failed = ไม่สามารถเชื่อมต่อ VPN
reason-ip-config = การกำหนดค่า IP ไม่ถูกต้อง
reason-timeout = การเชื่อมต่อหมดเวลา
reason-service-failed = บริการ VPN เริ่มทำงานไม่สำเร็จ
reason-no-secrets = ไม่ได้ระบุรหัสผ่าน
reason-login-failed = การยืนยันตัวตนล้มเหลว
detail-dns = DNS
detail-received = ได้รับ
detail-sent = ส่ง
detail-last-used = ใช้งานล่าสุด
duplicate = ทำสำเนา
priority = ลำดับความสำคัญการเชื่อมต่ออัตโนมัติ
metered = การเชื่อมต่อแบบจำกัดปริมาณ
metered-auto = อัตโนมัติ
metered-yes = ใช่
metered-no = ไม่
zone = โซนไฟร์วอลล์
split-tunnel = ใช้สำหรับเครือข่ายของตัวเองเท่านั้น
dns-search = โดเมนค้นหา DNS

# OpenConnect / OpenVPN / vpnc (added)
oc-general = ทั่วไป
oc-cert-auth = การยืนยันตัวตนด้วยใบรับรอง
oc-protocol = โปรโตคอล VPN
oc-ca-cert = ใบรับรอง CA
oc-proxy = พร็อกซี
oc-user-agent = User Agent
oc-reported-version = เวอร์ชันที่รายงาน
oc-reported-os = OS ที่รายงาน
oc-os-default = ค่าเริ่มต้น
oc-csd-trojan = อนุญาต Cisco Secure Desktop trojan
oc-csd-wrapper = สคริปต์ตัวห่อ CSD
oc-machine-cert = ใบรับรองเครื่อง
oc-private-key = คีย์ส่วนตัว
oc-user-cert = ใบรับรองผู้ใช้
oc-fsid = ใช้ FSID สำหรับวลีรหัสผ่านของคีย์
oc-prevent-invalid = ป้องกันไม่ให้ผู้ใช้ยอมรับใบรับรองที่ไม่ถูกต้องด้วยตนเอง
ovpn-conn-type = ประเภทการเชื่อมต่อ
ovpn-ct-tls = ใบรับรอง (TLS)
ovpn-ct-password = รหัสผ่าน
ovpn-ct-password-tls = รหัสผ่านพร้อมใบรับรอง (TLS)
ovpn-ct-static-key = คีย์แบบคงที่
ovpn-username = ชื่อผู้ใช้
ovpn-static-key = คีย์แบบคงที่
ovpn-key-dir = ทิศทางคีย์
ovpn-remote-ip = ที่อยู่ IP ระยะไกล
ovpn-local-ip = ที่อยู่ IP ในเครื่อง
ovpn-advanced = ขั้นสูง
ovpn-port = พอร์ตเกตเวย์
ovpn-tcp = ใช้การเชื่อมต่อ TCP
ovpn-cipher = การเข้ารหัส
ovpn-auth = การยืนยันตัวตน HMAC
vpnc-group = ชื่อกลุ่ม
vpnc-domain = โดเมน
vpnc-nat = NAT traversal
vpnc-pfs = Perfect Forward Secrecy
vpnc-dh = กลุ่ม DH
vpnc-vendor = ผู้จำหน่าย
vpnc-app-version = เวอร์ชันแอปพลิเคชัน
vpnc-single-des = เปิดใช้การเข้ารหัส DES เดี่ยวที่อ่อนแอ
