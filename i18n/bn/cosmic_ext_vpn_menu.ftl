app-title = COSMIC VPN মেনু

# Popup states
no-vpn-connections = কোনো VPN সংযোগ নেই
load-error = VPN সংযোগ লোড করা যায়নি: { $error }

# Connection status
vpn-status-connected = সংযুক্ত
vpn-status-connecting = সংযোগ করা হচ্ছে…
vpn-status-disconnecting = সংযোগ বিচ্ছিন্ন করা হচ্ছে…
vpn-status-disconnected = সংযোগ বিচ্ছিন্ন
vpn-status-unknown = অজানা

# Connection detail panel
detail-type = ধরন
detail-gateway = গেটওয়ে
detail-ipv4 = IPv4
detail-ipv6 = IPv6
detail-duration = সংযুক্ত ছিল
autoconnect = স্বয়ংক্রিয়ভাবে সংযোগ করুন

# Forget / delete
forget = ভুলে যান
confirm-forget = এই সংযোগটি ভুলে যাবেন?
delete = মুছুন
cancel = বাতিল

# Secret prompt
secret-prompt = { $name } এর জন্য শংসাপত্র লিখুন
connect = সংযোগ করুন

# Import
import = আমদানি করুন…

# Create / edit
add-vpn = VPN যোগ করুন
edit = সম্পাদনা…
select-vpn-type = VPN ধরন নির্বাচন করুন
new-vpn = নতুন { $kind } VPN
edit-vpn = { $kind } VPN সম্পাদনা করুন
name-placeholder = নাম
advanced-data = উন্নত (ডেটা)
key-placeholder = কী
value-placeholder = মান
add-field = ক্ষেত্র যোগ করুন
save = সংরক্ষণ করুন

# WireGuard editor
wg-private-key = ব্যক্তিগত কী
wg-address = ঠিকানা
wg-peer = পিয়ার
wg-peer-key = সর্বজনীন কী
wg-peer-endpoint = এন্ডপয়েন্ট
wg-allowed-ips = অনুমোদিত IP

# WireGuard / secrets (added)
dns-servers = DNS সার্ভার
wg-mtu = MTU
wg-peer-n = পিয়ার { $index }
wg-preshared-key = প্রি-শেয়ারড কী
wg-keepalive = স্থায়ী কিপঅ্যালাইভ (সে)
wg-add-peer = পিয়ার যোগ করুন
remove = সরান
secret-store = পাসওয়ার্ড সংরক্ষণ করুন
secret-ask = সর্বদা জিজ্ঞাসা করুন
secret-none = প্রয়োজন নেই

# Sync (backfilled)
notify-connected = VPN সংযুক্ত হয়েছে
notify-disconnected = VPN সংযোগ বিচ্ছিন্ন হয়েছে
notify-failed = VPN সংযোগ করা যায়নি
reason-ip-config = অবৈধ IP কনফিগারেশন
reason-timeout = সংযোগের সময় শেষ হয়েছে
reason-service-failed = VPN পরিষেবা শুরু করা যায়নি
reason-no-secrets = কোনো পাসওয়ার্ড দেওয়া হয়নি
reason-login-failed = প্রমাণীকরণ ব্যর্থ হয়েছে
detail-dns = DNS
detail-received = গৃহীত
detail-sent = প্রেরিত
detail-last-used = সর্বশেষ ব্যবহৃত
duplicate = অনুলিপি
priority = স্বয়ংক্রিয় সংযোগের অগ্রাধিকার
metered = মিটারযুক্ত
metered-auto = স্বয়ংক্রিয়
metered-yes = হ্যাঁ
metered-no = না
zone = ফায়ারওয়াল জোন
split-tunnel = শুধুমাত্র নিজস্ব নেটওয়ার্কের জন্য ব্যবহার করুন
dns-search = DNS অনুসন্ধান ডোমেইন

# OpenConnect / OpenVPN / vpnc (added)
oc-general = সাধারণ
oc-cert-auth = শংসাপত্র প্রমাণীকরণ
oc-protocol = VPN প্রোটোকল
oc-ca-cert = CA শংসাপত্র
oc-proxy = প্রক্সি
oc-user-agent = ইউজার এজেন্ট
oc-reported-version = রিপোর্ট করা সংস্করণ
oc-reported-os = রিপোর্ট করা OS
oc-os-default = ডিফল্ট
oc-csd-trojan = Cisco Secure Desktop ট্রোজান অনুমোদন করুন
oc-csd-wrapper = CSD র‍্যাপার স্ক্রিপ্ট
oc-machine-cert = মেশিন শংসাপত্র
oc-private-key = ব্যক্তিগত কী
oc-user-cert = ব্যবহারকারীর শংসাপত্র
oc-fsid = কী পাসফ্রেজের জন্য FSID ব্যবহার করুন
oc-prevent-invalid = ব্যবহারকারীকে ম্যানুয়ালি অবৈধ শংসাপত্র গ্রহণ করা থেকে বিরত রাখুন
ovpn-conn-type = সংযোগের ধরন
ovpn-ct-tls = শংসাপত্র (TLS)
ovpn-ct-password = পাসওয়ার্ড
ovpn-ct-password-tls = শংসাপত্রসহ পাসওয়ার্ড (TLS)
ovpn-ct-static-key = স্ট্যাটিক কী
ovpn-username = ব্যবহারকারীর নাম
ovpn-static-key = স্ট্যাটিক কী
ovpn-key-dir = কী দিক
ovpn-remote-ip = দূরবর্তী IP ঠিকানা
ovpn-local-ip = স্থানীয় IP ঠিকানা
ovpn-advanced = উন্নত
ovpn-port = গেটওয়ে পোর্ট
ovpn-tcp = একটি TCP সংযোগ ব্যবহার করুন
ovpn-cipher = সাইফার
ovpn-auth = HMAC প্রমাণীকরণ
vpnc-group = গ্রুপের নাম
vpnc-domain = ডোমেইন
vpnc-nat = NAT ট্রাভার্সাল
vpnc-pfs = পারফেক্ট ফরওয়ার্ড সিক্রেসি
vpnc-dh = DH গ্রুপ
vpnc-vendor = বিক্রেতা
vpnc-app-version = অ্যাপ্লিকেশন সংস্করণ
vpnc-single-des = দুর্বল একক DES এনক্রিপশন সক্ষম করুন
