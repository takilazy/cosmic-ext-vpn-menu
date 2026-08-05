app-title = COSMIC VPN -valikko

# Popup states
no-vpn-connections = Ei VPN-yhteyksiä
load-error = VPN-yhteyksien lataaminen epäonnistui: { $error }

# Connection status
vpn-status-connected = Yhdistetty
vpn-status-connecting = Yhdistetään…
vpn-status-disconnecting = Katkaistaan…
vpn-status-disconnected = Yhteys katkaistu
vpn-status-unknown = Tuntematon

# Connection detail panel
detail-type = Tyyppi
detail-gateway = Yhdyskäytävä
detail-ipv4 = IPv4
detail-ipv6 = IPv6
dns-servers = DNS-palvelimet
detail-duration = Yhdistetty
autoconnect = Yhdistä automaattisesti

# Forget / delete
forget = Unohda
confirm-forget = Unohdetaanko tämä yhteys?
delete = Poista
cancel = Peruuta

# Secret prompt
secret-prompt = Anna kohteen { $name } tunnistetiedot
connect = Yhdistä

# Import
import = Tuo…

# Create / edit
add-vpn = Lisää VPN
edit = Muokkaa…
select-vpn-type = Valitse VPN-tyyppi
new-vpn = Uusi { $kind } VPN
edit-vpn = Muokkaa { $kind } VPN:ää
name-placeholder = Nimi
advanced-data = Lisäasetukset (tiedot)
key-placeholder = Avain
value-placeholder = Arvo
add-field = Lisää kenttä
save = Tallenna

# WireGuard editor
wg-private-key = Yksityinen avain
wg-address = Osoite
wg-peer = Vertaisosapuoli
wg-peer-key = Julkinen avain
wg-peer-endpoint = Päätepiste
wg-allowed-ips = Sallitut IP:t

# WireGuard / secrets (added)
wg-mtu = MTU
wg-peer-n = Vertaisosapuoli { $index }
wg-preshared-key = Esijaettu avain
wg-keepalive = Pysyvä keepalive (s)
wg-add-peer = Lisää vertaisosapuoli
remove = Poista
secret-store = Tallenna salasana
secret-ask = Kysy aina
secret-none = Ei vaadita

# Sync (backfilled)
notify-connected = VPN yhdistetty
notify-disconnected = VPN katkaistu
notify-failed = VPN-yhteyden muodostaminen epäonnistui
reason-ip-config = virheellinen IP-asetus
reason-timeout = yhteys aikakatkaistiin
reason-service-failed = VPN-palvelun käynnistys epäonnistui
reason-no-secrets = salasanaa ei annettu
reason-login-failed = todennus epäonnistui
detail-dns = DNS
detail-received = Vastaanotettu
detail-sent = Lähetetty
detail-last-used = Viimeksi käytetty
duplicate = Monista
priority = Automaattiyhdistämisen prioriteetti
metered = Mitattu
metered-auto = Automaattinen
metered-yes = Kyllä
metered-no = Ei
zone = Palomuurivyöhyke
split-tunnel = Käytä vain omaan verkkoonsa
dns-search = DNS-hakutoimialueet

# OpenConnect / OpenVPN / vpnc (added)
oc-general = Yleiset
oc-cert-auth = Varmennetodennus
oc-protocol = VPN-protokolla
oc-ca-cert = CA-varmenne
oc-proxy = Välityspalvelin
oc-user-agent = Käyttäjäagentti
oc-reported-version = Ilmoitettu versio
oc-reported-os = Ilmoitettu käyttöjärjestelmä
oc-os-default = Oletus
oc-csd-trojan = Salli Cisco Secure Desktop -troijalainen
oc-csd-wrapper = CSD-kääreskripti
oc-machine-cert = Konevarmenne
oc-private-key = Yksityinen avain
oc-user-cert = Käyttäjän varmenne
oc-fsid = Käytä FSID:tä avaimen tunnuslauseena
oc-prevent-invalid = Estä käyttäjää hyväksymästä virheellisiä varmenteita käsin
ovpn-conn-type = Yhteystyyppi
ovpn-ct-tls = Varmenteet (TLS)
ovpn-ct-password = Salasana
ovpn-ct-password-tls = Salasana ja varmenteet (TLS)
ovpn-ct-static-key = Staattinen avain
ovpn-username = Käyttäjänimi
ovpn-static-key = Staattinen avain
ovpn-key-dir = Avaimen suunta
ovpn-remote-ip = Etä-IP-osoite
ovpn-local-ip = Paikallinen IP-osoite
ovpn-advanced = Lisäasetukset
ovpn-port = Yhdyskäytävän portti
ovpn-tcp = Käytä TCP-yhteyttä
ovpn-cipher = Salaus
ovpn-auth = HMAC-todennus
vpnc-group = Ryhmän nimi
vpnc-domain = Toimialue
vpnc-nat = NAT-läpäisy
vpnc-pfs = Täydellinen edeltävä salaisuus
vpnc-dh = DH-ryhmä
vpnc-vendor = Toimittaja
vpnc-app-version = Sovelluksen versio
vpnc-single-des = Ota käyttöön heikko yksittäinen DES-salaus
