app-title = COSMIC VPN मेनू

# Popup states
no-vpn-connections = कोई VPN कनेक्शन नहीं
load-error = VPN कनेक्शन लोड नहीं हो सके: { $error }

# Connection status
vpn-status-connected = कनेक्ट किया गया
vpn-status-connecting = कनेक्ट हो रहा है…
vpn-status-disconnecting = डिस्कनेक्ट हो रहा है…
vpn-status-disconnected = डिस्कनेक्ट किया गया
vpn-status-unknown = अज्ञात

# Connection detail panel
detail-type = प्रकार
detail-gateway = गेटवे
detail-ipv4 = IPv4
detail-ipv6 = IPv6
dns-servers = DNS सर्वर
detail-duration = कनेक्ट रहा
autoconnect = स्वचालित रूप से कनेक्ट करें

# Forget / delete
forget = भूलें
confirm-forget = इस कनेक्शन को भूलें?
delete = हटाएँ
cancel = रद्द करें

# Secret prompt
secret-prompt = { $name } के लिए क्रेडेंशियल दर्ज करें
connect = कनेक्ट करें

# Import
import = आयात करें…

# Create / edit
add-vpn = VPN जोड़ें
edit = संपादित करें…
select-vpn-type = VPN प्रकार चुनें
new-vpn = नया { $kind } VPN
edit-vpn = { $kind } VPN संपादित करें
name-placeholder = नाम
advanced-data = उन्नत (डेटा)
key-placeholder = कुंजी
value-placeholder = मान
add-field = फ़ील्ड जोड़ें
save = सहेजें

# WireGuard editor
wg-private-key = निजी कुंजी
wg-address = पता
wg-peer = पीयर
wg-peer-key = सार्वजनिक कुंजी
wg-peer-endpoint = एंडपॉइंट
wg-allowed-ips = अनुमत IP

# WireGuard / secrets (added)
wg-mtu = MTU
wg-peer-n = पीयर { $index }
wg-preshared-key = पूर्व-साझा कुंजी
wg-keepalive = स्थायी कीपअलाइव (से॰)
wg-add-peer = पीयर जोड़ें
remove = हटाएँ
secret-store = पासवर्ड सहेजें
secret-ask = हमेशा पूछें
secret-none = आवश्यक नहीं

# Sync (backfilled)
notify-connected = VPN कनेक्ट हुआ
notify-disconnected = VPN डिस्कनेक्ट हुआ
notify-failed = VPN कनेक्ट होने में विफल
reason-ip-config = अमान्य IP कॉन्फ़िगरेशन
reason-timeout = कनेक्शन का समय समाप्त हुआ
reason-service-failed = VPN सेवा शुरू होने में विफल रही
reason-no-secrets = कोई पासवर्ड नहीं दिया गया
reason-login-failed = प्रमाणीकरण विफल हुआ
detail-dns = DNS
detail-received = प्राप्त
detail-sent = भेजा गया
detail-last-used = अंतिम उपयोग
duplicate = डुप्लिकेट
priority = स्वतः कनेक्ट प्राथमिकता
metered = मीटर्ड
metered-auto = स्वचालित
metered-yes = हाँ
metered-no = नहीं
zone = फ़ायरवॉल ज़ोन
split-tunnel = केवल अपने नेटवर्क के लिए उपयोग करें
dns-search = DNS खोज डोमेन

# OpenConnect / OpenVPN / vpnc (added)
oc-general = सामान्य
oc-cert-auth = प्रमाणपत्र प्रमाणीकरण
oc-protocol = VPN प्रोटोकॉल
oc-ca-cert = CA प्रमाणपत्र
oc-proxy = प्रॉक्सी
oc-user-agent = उपयोगकर्ता एजेंट
oc-reported-version = रिपोर्ट किया गया संस्करण
oc-reported-os = रिपोर्ट किया गया OS
oc-os-default = डिफ़ॉल्ट
oc-csd-trojan = Cisco Secure Desktop ट्रोजन की अनुमति दें
oc-csd-wrapper = CSD रैपर स्क्रिप्ट
oc-machine-cert = मशीन प्रमाणपत्र
oc-private-key = निजी कुंजी
oc-user-cert = उपयोगकर्ता प्रमाणपत्र
oc-fsid = कुंजी पासफ़्रेज़ के लिए FSID का उपयोग करें
oc-prevent-invalid = उपयोगकर्ता को अमान्य प्रमाणपत्र मैन्युअल रूप से स्वीकार करने से रोकें
ovpn-conn-type = कनेक्शन प्रकार
ovpn-ct-tls = प्रमाणपत्र (TLS)
ovpn-ct-password = पासवर्ड
ovpn-ct-password-tls = प्रमाणपत्रों के साथ पासवर्ड (TLS)
ovpn-ct-static-key = स्थैतिक कुंजी
ovpn-username = उपयोगकर्ता नाम
ovpn-static-key = स्थैतिक कुंजी
ovpn-key-dir = कुंजी दिशा
ovpn-remote-ip = दूरस्थ IP पता
ovpn-local-ip = स्थानीय IP पता
ovpn-advanced = उन्नत
ovpn-port = गेटवे पोर्ट
ovpn-tcp = TCP कनेक्शन का उपयोग करें
ovpn-cipher = सिफर
ovpn-auth = HMAC प्रमाणीकरण
vpnc-group = समूह नाम
vpnc-domain = डोमेन
vpnc-nat = NAT ट्रैवर्सल
vpnc-pfs = परफेक्ट फ़ॉरवर्ड सीक्रेसी
vpnc-dh = DH समूह
vpnc-vendor = विक्रेता
vpnc-app-version = अनुप्रयोग संस्करण
vpnc-single-des = कमज़ोर सिंगल DES एन्क्रिप्शन सक्षम करें
