app-title = Menu VPN COSMIC

# Popup states
no-vpn-connections = Aucune connexion VPN
load-error = Impossible de charger les connexions VPN : { $error }

# Connection status
vpn-status-connected = Connecté
vpn-status-connecting = Connexion…
vpn-status-disconnecting = Déconnexion…
vpn-status-disconnected = Déconnecté
vpn-status-unknown = Inconnu

# Connection detail panel
detail-type = Type
detail-gateway = Passerelle
detail-ipv4 = IPv4
detail-ipv6 = IPv6
dns-servers = Serveurs DNS
detail-duration = Connecté depuis
autoconnect = Se connecter automatiquement

# Forget / delete
forget = Oublier
confirm-forget = Oublier cette connexion ?
delete = Supprimer
cancel = Annuler

# Secret prompt
secret-prompt = Saisir les identifiants pour { $name }
connect = Connecter

# Import
import = Importer…

# Create / edit
add-vpn = Ajouter un VPN
edit = Modifier…
select-vpn-type = Sélectionner le type de VPN
new-vpn = Nouveau VPN { $kind }
edit-vpn = Modifier le VPN { $kind }
name-placeholder = Nom
advanced-data = Avancé (données)
key-placeholder = Clé
value-placeholder = Valeur
add-field = Ajouter un champ
save = Enregistrer

# WireGuard editor
wg-private-key = Clé privée
wg-address = Adresse
wg-peer = Pair
wg-peer-key = Clé publique
wg-peer-endpoint = Point de terminaison
wg-allowed-ips = IP autorisées

# WireGuard / secrets (added)
wg-mtu = MTU
wg-peer-n = Pair { $index }
wg-preshared-key = Clé pré-partagée
wg-keepalive = Keepalive persistant (s)
wg-add-peer = Ajouter un pair
remove = Supprimer
secret-store = Enregistrer le mot de passe
secret-ask = Toujours demander
secret-none = Non requis

# Sync (backfilled)
notify-connected = VPN connecté
notify-disconnected = VPN déconnecté
notify-failed = Échec de la connexion VPN
reason-ip-config = configuration IP non valide
reason-timeout = délai de connexion dépassé
reason-service-failed = le service VPN n’a pas pu démarrer
reason-no-secrets = aucun mot de passe fourni
reason-login-failed = échec de l’authentification
detail-dns = DNS
detail-received = Reçu
detail-sent = Envoyé
detail-last-used = Dernière utilisation
duplicate = Dupliquer
priority = Priorité de connexion automatique
metered = Facturé à l’usage
metered-auto = Automatique
metered-yes = Oui
metered-no = Non
zone = Zone du pare-feu
split-tunnel = Utiliser uniquement pour son propre réseau
dns-search = Domaines de recherche DNS

# OpenConnect / OpenVPN / vpnc (added)
oc-general = Général
oc-cert-auth = Authentification par certificat
oc-protocol = Protocole VPN
oc-ca-cert = Certificat CA
oc-proxy = Proxy
oc-user-agent = Agent utilisateur
oc-reported-version = Version signalée
oc-reported-os = Système d’exploitation signalé
oc-os-default = Par défaut
oc-csd-trojan = Autoriser le cheval de Troie Cisco Secure Desktop
oc-csd-wrapper = Script d’enrobage CSD
oc-machine-cert = Certificat machine
oc-private-key = Clé privée
oc-user-cert = Certificat utilisateur
oc-fsid = Utiliser le FSID comme phrase secrète de la clé
oc-prevent-invalid = Empêcher l’utilisateur d’accepter manuellement des certificats non valides
ovpn-conn-type = Type de connexion
ovpn-ct-tls = Certificats (TLS)
ovpn-ct-password = Mot de passe
ovpn-ct-password-tls = Mot de passe avec certificats (TLS)
ovpn-ct-static-key = Clé statique
ovpn-username = Nom d’utilisateur
ovpn-static-key = Clé statique
ovpn-key-dir = Direction de la clé
ovpn-remote-ip = Adresse IP distante
ovpn-local-ip = Adresse IP locale
ovpn-advanced = Avancé
ovpn-port = Port de la passerelle
ovpn-tcp = Utiliser une connexion TCP
ovpn-cipher = Chiffrement
ovpn-auth = Authentification HMAC
vpnc-group = Nom du groupe
vpnc-domain = Domaine
vpnc-nat = Traversée NAT
vpnc-pfs = Confidentialité persistante parfaite
vpnc-dh = Groupe DH
vpnc-vendor = Fournisseur
vpnc-app-version = Version de l’application
vpnc-single-des = Activer le chiffrement DES simple faible
