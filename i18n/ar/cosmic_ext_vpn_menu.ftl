app-title = قائمة VPN لسطح مكتب COSMIC

# Popup states
no-vpn-connections = لا توجد اتصالات VPN
load-error = تعذّر تحميل اتصالات VPN: { $error }

# Connection status
vpn-status-connected = متصل
vpn-status-connecting = جارٍ الاتصال…
vpn-status-disconnecting = جارٍ قطع الاتصال…
vpn-status-disconnected = غير متصل
vpn-status-unknown = غير معروف

# Connection detail panel
detail-type = النوع
detail-gateway = البوابة
detail-ipv4 = IPv4
detail-ipv6 = IPv6
detail-duration = متصل منذ
autoconnect = الاتصال تلقائيًا

# Forget / delete
forget = نسيان
confirm-forget = هل تريد نسيان هذا الاتصال؟
delete = حذف
cancel = إلغاء

# Secret prompt
secret-prompt = أدخل بيانات الاعتماد لـ { $name }
connect = اتصال

# Import
import = استيراد…

# Create / edit
add-vpn = إضافة VPN
edit = تعديل…
select-vpn-type = اختر نوع VPN
new-vpn = ‏VPN { $kind } جديد
edit-vpn = تعديل ‏VPN { $kind }
name-placeholder = الاسم
advanced-data = متقدم (بيانات)
key-placeholder = المفتاح
value-placeholder = القيمة
add-field = إضافة حقل
save = حفظ

# WireGuard editor
wg-private-key = المفتاح الخاص
wg-address = العنوان
wg-peer = النظير
wg-peer-key = المفتاح العام
wg-peer-endpoint = نقطة النهاية
wg-allowed-ips = عناوين IP المسموح بها

# WireGuard / secrets (added)
dns-servers = خوادم DNS
wg-mtu = MTU
wg-peer-n = النظير { $index }
wg-preshared-key = المفتاح المشترك مسبقًا
wg-keepalive = إبقاء الاتصال حيًا (ث)
wg-add-peer = إضافة نظير
remove = إزالة
secret-store = تخزين كلمة المرور
secret-ask = اسأل دائمًا
secret-none = غير مطلوب

# Sync (backfilled)
notify-connected = تم الاتصال بـ VPN
notify-disconnected = تم قطع اتصال VPN
notify-failed = فشل اتصال VPN
reason-ip-config = إعدادات IP غير صالحة
reason-timeout = انتهت مهلة الاتصال
reason-service-failed = فشل بدء خدمة VPN
reason-no-secrets = لم يتم إدخال كلمة مرور
reason-login-failed = فشلت المصادقة
detail-dns = DNS
detail-received = المستلَم
detail-sent = المُرسَل
detail-last-used = آخر استخدام
duplicate = تكرار
priority = أولوية الاتصال التلقائي
metered = مُقنَّن
metered-auto = تلقائي
metered-yes = نعم
metered-no = لا
zone = منطقة الجدار الناري
split-tunnel = استخدامه لشبكته الخاصة فقط
dns-search = نطاقات بحث DNS

# OpenConnect / OpenVPN / vpnc (added)
oc-general = عام
oc-cert-auth = مصادقة الشهادة
oc-protocol = بروتوكول VPN
oc-ca-cert = شهادة CA
oc-proxy = وكيل
oc-user-agent = وكيل المستخدم
oc-reported-version = الإصدار المُبلَّغ عنه
oc-reported-os = نظام التشغيل المُبلَّغ عنه
oc-os-default = الافتراضي
oc-csd-trojan = السماح بحصان طروادة Cisco Secure Desktop
oc-csd-wrapper = برنامج تغليف CSD
oc-machine-cert = شهادة الجهاز
oc-private-key = المفتاح الخاص
oc-user-cert = شهادة المستخدم
oc-fsid = استخدام FSID لعبارة مرور المفتاح
oc-prevent-invalid = منع المستخدم من قبول الشهادات غير الصالحة يدويًا
ovpn-conn-type = نوع الاتصال
ovpn-ct-tls = شهادات (TLS)
ovpn-ct-password = كلمة المرور
ovpn-ct-password-tls = كلمة المرور مع شهادات (TLS)
ovpn-ct-static-key = مفتاح ثابت
ovpn-username = اسم المستخدم
ovpn-static-key = مفتاح ثابت
ovpn-key-dir = اتجاه المفتاح
ovpn-remote-ip = عنوان IP البعيد
ovpn-local-ip = عنوان IP المحلي
ovpn-advanced = متقدم
ovpn-port = منفذ البوابة
ovpn-tcp = استخدام اتصال TCP
ovpn-cipher = خوارزمية التشفير
ovpn-auth = مصادقة HMAC
vpnc-group = اسم المجموعة
vpnc-domain = النطاق
vpnc-nat = اجتياز NAT
vpnc-pfs = السرية التامة للتوجيه الأمامي
vpnc-dh = مجموعة DH
vpnc-vendor = المورّد
vpnc-app-version = إصدار التطبيق
vpnc-single-des = تفعيل تشفير DES المفرد الضعيف
