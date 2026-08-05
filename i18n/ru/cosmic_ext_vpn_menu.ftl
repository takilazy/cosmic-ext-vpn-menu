app-title = Меню VPN COSMIC

# Popup states
no-vpn-connections = Нет подключений VPN
load-error = Не удалось загрузить подключения VPN: { $error }

# Connection status
vpn-status-connected = Подключено
vpn-status-connecting = Подключение…
vpn-status-disconnecting = Отключение…
vpn-status-disconnected = Отключено
vpn-status-unknown = Неизвестно

# Connection detail panel
detail-type = Тип
detail-gateway = Шлюз
detail-ipv4 = IPv4
detail-ipv6 = IPv6
detail-duration = Подключено в течение
autoconnect = Подключаться автоматически

# Forget / delete
forget = Забыть
confirm-forget = Забыть это подключение?
delete = Удалить
cancel = Отмена

# Secret prompt
secret-prompt = Введите учётные данные для { $name }
connect = Подключить

# Import
import = Импорт…

# Create / edit
add-vpn = Добавить VPN
edit = Изменить…
select-vpn-type = Выберите тип VPN
new-vpn = Новый VPN { $kind }
edit-vpn = Изменить VPN { $kind }
name-placeholder = Название
advanced-data = Дополнительно (данные)
key-placeholder = Ключ
value-placeholder = Значение
add-field = Добавить поле
save = Сохранить

# WireGuard editor
wg-private-key = Закрытый ключ
wg-address = Адрес
wg-peer = Узел
wg-peer-key = Открытый ключ
wg-peer-endpoint = Конечная точка
wg-allowed-ips = Разрешённые IP
dns-servers = DNS-серверы

# WireGuard / secrets (added)
wg-mtu = MTU
wg-peer-n = Узел { $index }
wg-preshared-key = Общий ключ
wg-keepalive = Постоянный keepalive (с)
wg-add-peer = Добавить узел
remove = Удалить
secret-store = Сохранить пароль
secret-ask = Всегда спрашивать
secret-none = Не требуется

# Sync (backfilled)
notify-connected = VPN подключён
notify-disconnected = VPN отключён
notify-failed = Не удалось подключить VPN
reason-ip-config = неверная конфигурация IP
reason-timeout = время подключения истекло
reason-service-failed = не удалось запустить службу VPN
reason-no-secrets = пароль не указан
reason-login-failed = ошибка аутентификации
detail-dns = DNS
detail-received = Получено
detail-sent = Отправлено
detail-last-used = Последнее использование
duplicate = Дублировать
priority = Приоритет автоподключения
metered = Лимитное
metered-auto = Автоматически
metered-yes = Да
metered-no = Нет
zone = Зона брандмауэра
split-tunnel = Использовать только для своей сети
dns-search = Домены поиска DNS

# OpenConnect / OpenVPN / vpnc (added)
oc-general = Общие
oc-cert-auth = Аутентификация по сертификату
oc-protocol = Протокол VPN
oc-ca-cert = Сертификат CA
oc-proxy = Прокси
oc-user-agent = User Agent
oc-reported-version = Сообщаемая версия
oc-reported-os = Сообщаемая ОС
oc-os-default = По умолчанию
oc-csd-trojan = Разрешить троян Cisco Secure Desktop
oc-csd-wrapper = Сценарий-обёртка CSD
oc-machine-cert = Сертификат машины
oc-private-key = Закрытый ключ
oc-user-cert = Сертификат пользователя
oc-fsid = Использовать FSID для парольной фразы ключа
oc-prevent-invalid = Запретить пользователю вручную принимать недействительные сертификаты
ovpn-conn-type = Тип подключения
ovpn-ct-tls = Сертификаты (TLS)
ovpn-ct-password = Пароль
ovpn-ct-password-tls = Пароль с сертификатами (TLS)
ovpn-ct-static-key = Статический ключ
ovpn-username = Имя пользователя
ovpn-static-key = Статический ключ
ovpn-key-dir = Направление ключа
ovpn-remote-ip = Удалённый IP-адрес
ovpn-local-ip = Локальный IP-адрес
ovpn-advanced = Дополнительно
ovpn-port = Порт шлюза
ovpn-tcp = Использовать TCP-подключение
ovpn-cipher = Шифр
ovpn-auth = Аутентификация HMAC
vpnc-group = Имя группы
vpnc-domain = Домен
vpnc-nat = Обход NAT
vpnc-pfs = Perfect Forward Secrecy
vpnc-dh = Группа DH
vpnc-vendor = Поставщик
vpnc-app-version = Версия приложения
vpnc-single-des = Включить слабое шифрование single DES
