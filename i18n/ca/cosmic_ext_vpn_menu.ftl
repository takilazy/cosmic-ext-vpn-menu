app-title = Menú VPN de COSMIC

# Popup states
no-vpn-connections = No hi ha connexions VPN
load-error = No s'han pogut carregar les connexions VPN: { $error }

# Connection status
vpn-status-connected = Connectat
vpn-status-connecting = S'està connectant…
vpn-status-disconnecting = S'està desconnectant…
vpn-status-disconnected = Desconnectat
vpn-status-unknown = Desconegut

# Connection detail panel
detail-type = Tipus
detail-gateway = Passarel·la
detail-ipv4 = IPv4
detail-ipv6 = IPv6
detail-duration = Connectat des de fa
autoconnect = Connecta automàticament

# Forget / delete
forget = Oblida
confirm-forget = Voleu oblidar aquesta connexió?
delete = Suprimeix
cancel = Cancel·la

# Secret prompt
secret-prompt = Introduïu les credencials per a { $name }
connect = Connecta

# Import
import = Importa…

# Create / edit
add-vpn = Afegeix una VPN
edit = Edita…
select-vpn-type = Seleccioneu el tipus de VPN
new-vpn = VPN { $kind } nova
edit-vpn = Edita la VPN { $kind }
name-placeholder = Nom
advanced-data = Avançat (dades)
key-placeholder = Clau
value-placeholder = Valor
add-field = Afegeix un camp
save = Desa

# WireGuard editor
wg-private-key = Clau privada
wg-address = Adreça
wg-peer = Igual
wg-peer-key = Clau pública
wg-peer-endpoint = Punt final
wg-allowed-ips = IP permeses

# WireGuard / secrets (added)
dns-servers = Servidors DNS
wg-mtu = MTU
wg-peer-n = Igual { $index }
wg-preshared-key = Clau precompartida
wg-keepalive = Manteniment de connexió persistent (s)
wg-add-peer = Afegeix un igual
remove = Elimina
secret-store = Desa la contrasenya
secret-ask = Pregunta sempre
secret-none = No cal

# Sync (backfilled)
notify-connected = VPN connectada
notify-disconnected = VPN desconnectada
notify-failed = No s'ha pogut connectar la VPN
reason-ip-config = configuració IP no vàlida
reason-timeout = s'ha esgotat el temps d'espera de la connexió
reason-service-failed = el servei VPN no s'ha pogut iniciar
reason-no-secrets = no s'ha proporcionat cap contrasenya
reason-login-failed = ha fallat l'autenticació
detail-dns = DNS
detail-received = Rebut
detail-sent = Enviat
detail-last-used = Darrer ús
duplicate = Duplica
priority = Prioritat de connexió automàtica
metered = Amb mesura d'ús
metered-auto = Automàtic
metered-yes = Sí
metered-no = No
zone = Zona del tallafoc
split-tunnel = Usa només per a la seva pròpia xarxa
dns-search = Dominis de cerca DNS

# OpenConnect / OpenVPN / vpnc (added)
oc-general = General
oc-cert-auth = Autenticació per certificat
oc-protocol = Protocol de VPN
oc-ca-cert = Certificat de CA
oc-proxy = Servidor intermediari
oc-user-agent = Agent d'usuari
oc-reported-version = Versió informada
oc-reported-os = Sistema operatiu informat
oc-os-default = Per defecte
oc-csd-trojan = Permet el troià Cisco Secure Desktop
oc-csd-wrapper = Script d'embolcall CSD
oc-machine-cert = Certificat de màquina
oc-private-key = Clau privada
oc-user-cert = Certificat d'usuari
oc-fsid = Usa el FSID per a la frase de contrasenya de la clau
oc-prevent-invalid = Impedeix que l'usuari accepti manualment certificats no vàlids
ovpn-conn-type = Tipus de connexió
ovpn-ct-tls = Certificats (TLS)
ovpn-ct-password = Contrasenya
ovpn-ct-password-tls = Contrasenya amb certificats (TLS)
ovpn-ct-static-key = Clau estàtica
ovpn-username = Nom d'usuari
ovpn-static-key = Clau estàtica
ovpn-key-dir = Direcció de la clau
ovpn-remote-ip = Adreça IP remota
ovpn-local-ip = Adreça IP local
ovpn-advanced = Avançat
ovpn-port = Port de la passarel·la
ovpn-tcp = Usa una connexió TCP
ovpn-cipher = Xifratge
ovpn-auth = Autenticació HMAC
vpnc-group = Nom del grup
vpnc-domain = Domini
vpnc-nat = Traversament de NAT
vpnc-pfs = Confidencialitat directa perfecta
vpnc-dh = Grup DH
vpnc-vendor = Proveïdor
vpnc-app-version = Versió de l'aplicació
vpnc-single-des = Habilita el xifratge feble de DES simple
