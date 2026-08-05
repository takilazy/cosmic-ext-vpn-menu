app-title = COSMIC VPN-meny

# Popup states
no-vpn-connections = Inga VPN-anslutningar
load-error = Det gick inte att läsa in VPN-anslutningar: { $error }

# Connection status
vpn-status-connected = Ansluten
vpn-status-connecting = Ansluter…
vpn-status-disconnecting = Kopplar från…
vpn-status-disconnected = Frånkopplad
vpn-status-unknown = Okänd

# Connection detail panel
detail-type = Typ
detail-gateway = Gateway
detail-ipv4 = IPv4
detail-ipv6 = IPv6
detail-duration = Ansluten i
autoconnect = Anslut automatiskt

# Forget / delete
forget = Glöm
confirm-forget = Glömma den här anslutningen?
delete = Ta bort
cancel = Avbryt

# Secret prompt
secret-prompt = Ange autentiseringsuppgifter för { $name }
connect = Anslut

# Import
import = Importera…

# Create / edit
add-vpn = Lägg till VPN
edit = Redigera…
select-vpn-type = Välj VPN-typ
new-vpn = Nytt { $kind }-VPN
edit-vpn = Redigera { $kind }-VPN
name-placeholder = Namn
advanced-data = Avancerat (data)
key-placeholder = Nyckel
value-placeholder = Värde
add-field = Lägg till fält
save = Spara

# WireGuard editor
wg-private-key = Privat nyckel
wg-address = Adress
wg-peer = Peer
wg-peer-key = Publik nyckel
wg-peer-endpoint = Slutpunkt
wg-allowed-ips = Tillåtna IP-adresser

# WireGuard / secrets (added)
dns-servers = DNS-servrar
wg-mtu = MTU
wg-peer-n = Peer { $index }
wg-preshared-key = Fördelad nyckel
wg-keepalive = Beständig keepalive (s)
wg-add-peer = Lägg till peer
remove = Ta bort
secret-store = Spara lösenord
secret-ask = Fråga alltid
secret-none = Krävs inte

# Sync (backfilled)
notify-connected = VPN ansluten
notify-disconnected = VPN frånkopplad
notify-failed = VPN kunde inte ansluta
reason-ip-config = ogiltig IP-konfiguration
reason-timeout = anslutningen tog för lång tid
reason-service-failed = VPN-tjänsten kunde inte startas
reason-no-secrets = inget lösenord angavs
reason-login-failed = autentiseringen misslyckades
detail-dns = DNS
detail-received = Mottaget
detail-sent = Skickat
detail-last-used = Senast använd
duplicate = Duplicera
priority = Prioritet för automatisk anslutning
metered = Kvotmätt
metered-auto = Automatisk
metered-yes = Ja
metered-no = Nej
zone = Brandväggszon
split-tunnel = Använd endast för sitt eget nätverk
dns-search = DNS-sökdomäner

# OpenConnect / OpenVPN / vpnc (added)
oc-general = Allmänt
oc-cert-auth = Certifikatautentisering
oc-protocol = VPN-protokoll
oc-ca-cert = CA-certifikat
oc-proxy = Proxy
oc-user-agent = Användaragent
oc-reported-version = Rapporterad version
oc-reported-os = Rapporterat OS
oc-os-default = Standard
oc-csd-trojan = Tillåt Cisco Secure Desktop-trojan
oc-csd-wrapper = CSD-omslagsskript
oc-machine-cert = Maskincertifikat
oc-private-key = Privat nyckel
oc-user-cert = Användarcertifikat
oc-fsid = Använd FSID för nyckellösenfras
oc-prevent-invalid = Hindra användare från att manuellt godkänna ogiltiga certifikat
ovpn-conn-type = Anslutningstyp
ovpn-ct-tls = Certifikat (TLS)
ovpn-ct-password = Lösenord
ovpn-ct-password-tls = Lösenord med certifikat (TLS)
ovpn-ct-static-key = Statisk nyckel
ovpn-username = Användarnamn
ovpn-static-key = Statisk nyckel
ovpn-key-dir = Nyckelriktning
ovpn-remote-ip = Fjärr-IP-adress
ovpn-local-ip = Lokal IP-adress
ovpn-advanced = Avancerat
ovpn-port = Gateway-port
ovpn-tcp = Använd en TCP-anslutning
ovpn-cipher = Chiffer
ovpn-auth = HMAC-autentisering
vpnc-group = Gruppnamn
vpnc-domain = Domän
vpnc-nat = NAT-traversering
vpnc-pfs = Perfect Forward Secrecy
vpnc-dh = DH-grupp
vpnc-vendor = Leverantör
vpnc-app-version = Programversion
vpnc-single-des = Aktivera svag enkel DES-kryptering
