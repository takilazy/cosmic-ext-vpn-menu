app-title = Menú VPN de COSMIC

# Popup states
no-vpn-connections = No hay conexiones VPN
load-error = No se pudieron cargar las conexiones VPN: { $error }

# Connection status
vpn-status-connected = Conectado
vpn-status-connecting = Conectando…
vpn-status-disconnecting = Desconectando…
vpn-status-disconnected = Desconectado
vpn-status-unknown = Desconocido

# Connection detail panel
detail-type = Tipo
detail-gateway = Puerta de enlace
detail-ipv4 = IPv4
detail-ipv6 = IPv6
detail-duration = Conectado durante
autoconnect = Conectar automáticamente

# Forget / delete
forget = Olvidar
confirm-forget = ¿Olvidar esta conexión?
delete = Eliminar
cancel = Cancelar

# Secret prompt
secret-prompt = Ingresa las credenciales para { $name }
connect = Conectar

# Import
import = Importar…

# Create / edit
add-vpn = Agregar VPN
edit = Editar…
select-vpn-type = Selecciona el tipo de VPN
new-vpn = Nueva VPN { $kind }
edit-vpn = Editar VPN { $kind }
name-placeholder = Nombre
advanced-data = Avanzado (datos)
key-placeholder = Clave
value-placeholder = Valor
add-field = Agregar campo
save = Guardar

# WireGuard editor
wg-private-key = Clave privada
wg-address = Dirección
wg-peer = Par
wg-peer-key = Clave pública
wg-peer-endpoint = Extremo
wg-allowed-ips = IP permitidas

# WireGuard / secrets (added)
dns-servers = Servidores DNS
wg-mtu = MTU
wg-peer-n = Par { $index }
wg-preshared-key = Clave precompartida
wg-keepalive = Keepalive persistente (s)
wg-add-peer = Agregar par
remove = Quitar
secret-store = Guardar contraseña
secret-ask = Preguntar siempre
secret-none = No requerido

# Sync (backfilled)
notify-connected = VPN conectada
notify-disconnected = VPN desconectada
notify-failed = No se pudo conectar la VPN
reason-ip-config = configuración de IP no válida
reason-timeout = se agotó el tiempo de conexión
reason-service-failed = el servicio VPN no pudo iniciarse
reason-no-secrets = no se proporcionó contraseña
reason-login-failed = error de autenticación
detail-dns = DNS
detail-received = Recibido
detail-sent = Enviado
detail-last-used = Último uso
duplicate = Duplicar
priority = Prioridad de conexión automática
metered = De uso medido
metered-auto = Automático
metered-yes = Sí
metered-no = No
zone = Zona de firewall
split-tunnel = Usar solo para su propia red
dns-search = Dominios de búsqueda DNS

# OpenConnect / OpenVPN / vpnc (added)
oc-general = General
oc-cert-auth = Autenticación por certificado
oc-protocol = Protocolo VPN
oc-ca-cert = Certificado CA
oc-proxy = Proxy
oc-user-agent = Agente de usuario
oc-reported-version = Versión informada
oc-reported-os = Sistema operativo informado
oc-os-default = Predeterminado
oc-csd-trojan = Permitir el troyano Cisco Secure Desktop
oc-csd-wrapper = Script contenedor de CSD
oc-machine-cert = Certificado de máquina
oc-private-key = Clave privada
oc-user-cert = Certificado de usuario
oc-fsid = Usar FSID para la frase de contraseña de la clave
oc-prevent-invalid = Impedir que el usuario acepte manualmente certificados no válidos
ovpn-conn-type = Tipo de conexión
ovpn-ct-tls = Certificados (TLS)
ovpn-ct-password = Contraseña
ovpn-ct-password-tls = Contraseña con certificados (TLS)
ovpn-ct-static-key = Clave estática
ovpn-username = Nombre de usuario
ovpn-static-key = Clave estática
ovpn-key-dir = Dirección de clave
ovpn-remote-ip = Dirección IP remota
ovpn-local-ip = Dirección IP local
ovpn-advanced = Avanzado
ovpn-port = Puerto de la puerta de enlace
ovpn-tcp = Usar una conexión TCP
ovpn-cipher = Cifrado
ovpn-auth = Autenticación HMAC
vpnc-group = Nombre del grupo
vpnc-domain = Dominio
vpnc-nat = Cruce de NAT
vpnc-pfs = Perfect Forward Secrecy
vpnc-dh = Grupo DH
vpnc-vendor = Proveedor
vpnc-app-version = Versión de la aplicación
vpnc-single-des = Habilitar el cifrado débil single DES
