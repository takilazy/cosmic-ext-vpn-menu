app-title = Меню VPN COSMIC

# Popup states
no-vpn-connections = Немає з’єднань VPN
load-error = Не вдалося завантажити з’єднання VPN: { $error }

# Connection status
vpn-status-connected = З’єднано
vpn-status-connecting = З’єднання…
vpn-status-disconnecting = Роз’єднання…
vpn-status-disconnected = Роз’єднано
vpn-status-unknown = Невідомо

# Connection detail panel
detail-type = Тип
detail-gateway = Шлюз
detail-ipv4 = IPv4
detail-ipv6 = IPv6
detail-duration = З’єднано протягом
autoconnect = З’єднуватися автоматично

# Forget / delete
forget = Забути
confirm-forget = Забути це з’єднання?
delete = Видалити
cancel = Скасувати

# Secret prompt
secret-prompt = Введіть облікові дані для { $name }
connect = З’єднати

# Import
import = Імпорт…

# Create / edit
add-vpn = Додати VPN
edit = Змінити…
select-vpn-type = Виберіть тип VPN
new-vpn = Новий VPN { $kind }
edit-vpn = Змінити VPN { $kind }
name-placeholder = Назва
advanced-data = Додатково (дані)
key-placeholder = Ключ
value-placeholder = Значення
add-field = Додати поле
save = Зберегти

# WireGuard editor
wg-private-key = Закритий ключ
wg-address = Адреса
wg-peer = Вузол
wg-peer-key = Відкритий ключ
wg-peer-endpoint = Кінцева точка
wg-allowed-ips = Дозволені IP

# WireGuard / secrets (added)
dns-servers = Сервери DNS
wg-mtu = MTU
wg-peer-n = Вузол { $index }
wg-preshared-key = Спільний ключ
wg-keepalive = Постійний keepalive (с)
wg-add-peer = Додати вузол
remove = Вилучити
secret-store = Зберегти пароль
secret-ask = Завжди запитувати
secret-none = Не потрібно

# Sync (backfilled)
notify-connected = VPN з’єднано
notify-disconnected = VPN роз’єднано
notify-failed = Не вдалося з’єднати VPN
reason-ip-config = недійсна конфігурація IP
reason-timeout = час очікування з’єднання вичерпано
reason-service-failed = не вдалося запустити службу VPN
reason-no-secrets = пароль не надано
reason-login-failed = помилка автентифікації
detail-dns = DNS
detail-received = Отримано
detail-sent = Надіслано
detail-last-used = Востаннє використано
duplicate = Дублювати
priority = Пріоритет автоз’єднання
metered = Тарифне
metered-auto = Автоматично
metered-yes = Так
metered-no = Ні
zone = Зона брандмауера
split-tunnel = Використовувати лише для власної мережі
dns-search = Домени пошуку DNS

# OpenConnect / OpenVPN / vpnc (added)
oc-general = Загальне
oc-cert-auth = Автентифікація за сертифікатом
oc-protocol = Протокол VPN
oc-ca-cert = Сертифікат CA
oc-proxy = Проксі
oc-user-agent = User Agent
oc-reported-version = Повідомлена версія
oc-reported-os = Повідомлена ОС
oc-os-default = Типово
oc-csd-trojan = Дозволити троян Cisco Secure Desktop
oc-csd-wrapper = Скрипт-обгортка CSD
oc-machine-cert = Сертифікат комп’ютера
oc-private-key = Закритий ключ
oc-user-cert = Сертифікат користувача
oc-fsid = Використовувати FSID для парольної фрази ключа
oc-prevent-invalid = Забороняти користувачу вручну приймати недійсні сертифікати
ovpn-conn-type = Тип з’єднання
ovpn-ct-tls = Сертифікати (TLS)
ovpn-ct-password = Пароль
ovpn-ct-password-tls = Пароль із сертифікатами (TLS)
ovpn-ct-static-key = Статичний ключ
ovpn-username = Ім’я користувача
ovpn-static-key = Статичний ключ
ovpn-key-dir = Напрямок ключа
ovpn-remote-ip = Віддалена IP-адреса
ovpn-local-ip = Локальна IP-адреса
ovpn-advanced = Додатково
ovpn-port = Порт шлюзу
ovpn-tcp = Використовувати з’єднання TCP
ovpn-cipher = Шифр
ovpn-auth = Автентифікація HMAC
vpnc-group = Назва групи
vpnc-domain = Домен
vpnc-nat = Обхід NAT
vpnc-pfs = Perfect Forward Secrecy
vpnc-dh = Група DH
vpnc-vendor = Постачальник
vpnc-app-version = Версія застосунку
vpnc-single-des = Увімкнути слабке шифрування single DES
