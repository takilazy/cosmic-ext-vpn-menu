app-title = Μενού VPN COSMIC

# Popup states
no-vpn-connections = Δεν υπάρχουν συνδέσεις VPN
load-error = Δεν ήταν δυνατή η φόρτωση των συνδέσεων VPN: { $error }

# Connection status
vpn-status-connected = Συνδεδεμένο
vpn-status-connecting = Σύνδεση…
vpn-status-disconnecting = Αποσύνδεση…
vpn-status-disconnected = Αποσυνδεδεμένο
vpn-status-unknown = Άγνωστο

# Connection detail panel
detail-type = Τύπος
detail-gateway = Πύλη
detail-ipv4 = IPv4
detail-ipv6 = IPv6
detail-duration = Συνδεδεμένο για
autoconnect = Αυτόματη σύνδεση

# Forget / delete
forget = Παράλειψη
confirm-forget = Παράλειψη αυτής της σύνδεσης;
delete = Διαγραφή
cancel = Άκυρο

# Secret prompt
secret-prompt = Εισαγάγετε διαπιστευτήρια για { $name }
connect = Σύνδεση

# Import
import = Εισαγωγή…

# Create / edit
add-vpn = Προσθήκη VPN
edit = Επεξεργασία…
select-vpn-type = Επιλογή τύπου VPN
new-vpn = Νέο VPN { $kind }
edit-vpn = Επεξεργασία VPN { $kind }
name-placeholder = Όνομα
advanced-data = Για προχωρημένους (δεδομένα)
key-placeholder = Κλειδί
value-placeholder = Τιμή
add-field = Προσθήκη πεδίου
save = Αποθήκευση

# WireGuard editor
wg-private-key = Ιδιωτικό κλειδί
wg-address = Διεύθυνση
wg-peer = Ομότιμος
wg-peer-key = Δημόσιο κλειδί
wg-peer-endpoint = Τελικό σημείο
wg-allowed-ips = Επιτρεπόμενες IP

# WireGuard / secrets (added)
dns-servers = Διακομιστές DNS
wg-mtu = MTU
wg-peer-n = Ομότιμος { $index }
wg-preshared-key = Προμοιρασμένο κλειδί
wg-keepalive = Επίμονο keepalive (δευτ.)
wg-add-peer = Προσθήκη ομότιμου
remove = Αφαίρεση
secret-store = Αποθήκευση κωδικού πρόσβασης
secret-ask = Να ερωτάται πάντα
secret-none = Δεν απαιτείται

# Sync (backfilled)
notify-connected = Το VPN συνδέθηκε
notify-disconnected = Το VPN αποσυνδέθηκε
notify-failed = Αποτυχία σύνδεσης VPN
reason-ip-config = μη έγκυρη διαμόρφωση IP
reason-timeout = λήξη χρονικού ορίου σύνδεσης
reason-service-failed = Η υπηρεσία VPN απέτυχε να εκκινήσει
reason-no-secrets = δεν δόθηκε κωδικός πρόσβασης
reason-login-failed = η ταυτοποίηση απέτυχε
detail-dns = DNS
detail-received = Ελήφθησαν
detail-sent = Απεστάλησαν
detail-last-used = Τελευταία χρήση
duplicate = Διπλότυπο
priority = Προτεραιότητα αυτόματης σύνδεσης
metered = Με χρέωση
metered-auto = Αυτόματο
metered-yes = Ναι
metered-no = Όχι
zone = Ζώνη τείχους προστασίας
split-tunnel = Χρήση μόνο για το δικό του δίκτυο
dns-search = Τομείς αναζήτησης DNS

# OpenConnect / OpenVPN / vpnc (added)
oc-general = Γενικά
oc-cert-auth = Ταυτοποίηση με πιστοποιητικό
oc-protocol = Πρωτόκολλο VPN
oc-ca-cert = Πιστοποιητικό CA
oc-proxy = Διαμεσολαβητής
oc-user-agent = User Agent
oc-reported-version = Αναφερόμενη έκδοση
oc-reported-os = Αναφερόμενο λειτουργικό σύστημα
oc-os-default = Προεπιλογή
oc-csd-trojan = Να επιτρέπεται ο δούρειος ίππος Cisco Secure Desktop
oc-csd-wrapper = Σενάριο περιτυλίγματος CSD
oc-machine-cert = Πιστοποιητικό μηχανήματος
oc-private-key = Ιδιωτικό κλειδί
oc-user-cert = Πιστοποιητικό χρήστη
oc-fsid = Χρήση FSID για τη φράση πρόσβασης του κλειδιού
oc-prevent-invalid = Αποτροπή χειροκίνητης αποδοχής μη έγκυρων πιστοποιητικών από τον χρήστη
ovpn-conn-type = Τύπος σύνδεσης
ovpn-ct-tls = Πιστοποιητικά (TLS)
ovpn-ct-password = Κωδικός πρόσβασης
ovpn-ct-password-tls = Κωδικός πρόσβασης με πιστοποιητικά (TLS)
ovpn-ct-static-key = Στατικό κλειδί
ovpn-username = Όνομα χρήστη
ovpn-static-key = Στατικό κλειδί
ovpn-key-dir = Κατεύθυνση κλειδιού
ovpn-remote-ip = Απομακρυσμένη διεύθυνση IP
ovpn-local-ip = Τοπική διεύθυνση IP
ovpn-advanced = Για προχωρημένους
ovpn-port = Θύρα πύλης
ovpn-tcp = Χρήση σύνδεσης TCP
ovpn-cipher = Κρυπτογράφηση
ovpn-auth = Ταυτοποίηση HMAC
vpnc-group = Όνομα ομάδας
vpnc-domain = Τομέας
vpnc-nat = Διέλευση NAT
vpnc-pfs = Perfect Forward Secrecy
vpnc-dh = Ομάδα DH
vpnc-vendor = Προμηθευτής
vpnc-app-version = Έκδοση εφαρμογής
vpnc-single-des = Ενεργοποίηση αδύναμης κρυπτογράφησης single DES
