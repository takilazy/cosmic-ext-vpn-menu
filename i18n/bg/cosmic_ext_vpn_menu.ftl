app-title = Меню за VPN на COSMIC

# Popup states
no-vpn-connections = Няма връзки VPN
load-error = Връзките VPN не могат да се заредят: { $error }

# Connection status
vpn-status-connected = Свързано
vpn-status-connecting = Свързване…
vpn-status-disconnecting = Прекъсване…
vpn-status-disconnected = Прекъснато
vpn-status-unknown = Неизвестно

# Connection detail panel
detail-type = Вид
detail-gateway = Шлюз
detail-ipv4 = IPv4
detail-ipv6 = IPv6
detail-duration = Свързано от
autoconnect = Автоматично свързване

# Forget / delete
forget = Забравяне
confirm-forget = Да се забрави ли тази връзка?
delete = Изтриване
cancel = Отказ

# Secret prompt
secret-prompt = Въведете идентификационни данни за { $name }
connect = Свързване

# Import
import = Внасяне…

# Create / edit
add-vpn = Добавяне на VPN
edit = Редактиране…
select-vpn-type = Изберете вид VPN
new-vpn = Нов VPN { $kind }
edit-vpn = Редактиране на VPN { $kind }
name-placeholder = Име
advanced-data = Разширени (данни)
key-placeholder = Ключ
value-placeholder = Стойност
add-field = Добавяне на поле
save = Запазване

# WireGuard editor
wg-private-key = Частен ключ
wg-address = Адрес
wg-peer = Партньор
wg-peer-key = Публичен ключ
wg-peer-endpoint = Крайна точка
wg-allowed-ips = Разрешени IP

# WireGuard / secrets (added)
dns-servers = DNS сървъри
wg-mtu = MTU
wg-peer-n = Партньор { $index }
wg-preshared-key = Предварително споделен ключ
wg-keepalive = Постоянно поддържане на връзката (сек)
wg-add-peer = Добавяне на партньор
remove = Премахване
secret-store = Запазване на паролата
secret-ask = Винаги да се пита
secret-none = Не се изисква

# Sync (backfilled)
notify-connected = VPN е свързан
notify-disconnected = VPN е прекъснат
notify-failed = Неуспешно свързване към VPN
reason-ip-config = невалидна настройка на IP
reason-timeout = времето за връзка изтече
reason-service-failed = услугата за VPN не успя да се стартира
reason-no-secrets = не е въведена парола
reason-login-failed = удостоверяването е неуспешно
detail-dns = DNS
detail-received = Получени
detail-sent = Изпратени
detail-last-used = Последно използвано
duplicate = Дублиране
priority = Приоритет на автоматичното свързване
metered = С отчитане на трафика
metered-auto = Автоматично
metered-yes = Да
metered-no = Не
zone = Зона на защитната стена
split-tunnel = Използване само за собствената мрежа
dns-search = Домейни за търсене в DNS

# OpenConnect / OpenVPN / vpnc (added)
oc-general = Общи
oc-cert-auth = Удостоверяване със сертификат
oc-protocol = Протокол на VPN
oc-ca-cert = Сертификат на CA
oc-proxy = Сървър-посредник
oc-user-agent = Потребителски агент
oc-reported-version = Докладвана версия
oc-reported-os = Докладвана ОС
oc-os-default = По подразбиране
oc-csd-trojan = Разрешаване на троянеца Cisco Secure Desktop
oc-csd-wrapper = Скрипт-обвивка за CSD
oc-machine-cert = Сертификат на машината
oc-private-key = Частен ключ
oc-user-cert = Потребителски сертификат
oc-fsid = Използване на FSID за паролната фраза на ключа
oc-prevent-invalid = Забрана потребителят ръчно да приема невалидни сертификати
ovpn-conn-type = Вид на връзката
ovpn-ct-tls = Сертификати (TLS)
ovpn-ct-password = Парола
ovpn-ct-password-tls = Парола със сертификати (TLS)
ovpn-ct-static-key = Статичен ключ
ovpn-username = Потребителско име
ovpn-static-key = Статичен ключ
ovpn-key-dir = Посока на ключа
ovpn-remote-ip = Отдалечен IP адрес
ovpn-local-ip = Локален IP адрес
ovpn-advanced = Разширени
ovpn-port = Порт на шлюза
ovpn-tcp = Използване на връзка по TCP
ovpn-cipher = Шифър
ovpn-auth = Удостоверяване с HMAC
vpnc-group = Име на групата
vpnc-domain = Домейн
vpnc-nat = Преминаване през NAT
vpnc-pfs = Перфектна права секретност
vpnc-dh = Група DH
vpnc-vendor = Производител
vpnc-app-version = Версия на приложението
vpnc-single-des = Разрешаване на слабото единично DES шифроване
