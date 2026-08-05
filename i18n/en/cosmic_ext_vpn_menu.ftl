app-title = COSMIC VPN Menu

# Popup states
no-vpn-connections = No VPN connections
load-error = Couldn't load VPN connections: { $error }

# Desktop notifications
notify-connected = VPN connected
notify-disconnected = VPN disconnected
notify-failed = VPN failed to connect

# Failure reasons
reason-ip-config = invalid IP configuration
reason-timeout = connection timed out
reason-service-failed = the VPN service failed to start
reason-no-secrets = no password provided
reason-login-failed = authentication failed

# Connection status
vpn-status-connected = Connected
vpn-status-connecting = Connecting…
vpn-status-disconnecting = Disconnecting…
vpn-status-disconnected = Disconnected
vpn-status-unknown = Unknown

# Connection detail panel
detail-type = Type
detail-gateway = Gateway
detail-ipv4 = IPv4
detail-ipv6 = IPv6
detail-dns = DNS
detail-received = Received
detail-sent = Sent
detail-duration = Connected for
detail-last-used = Last used
autoconnect = Connect automatically

# Forget / delete
duplicate = Duplicate
forget = Forget
confirm-forget = Forget this connection?
delete = Delete
cancel = Cancel

# Secret prompt
secret-prompt = Enter credentials for { $name }
connect = Connect

# Import
import = Import…

# Create / edit
add-vpn = Add VPN
edit = Edit…
select-vpn-type = Select VPN type
new-vpn = New { $kind } VPN
edit-vpn = Edit { $kind } VPN
name-placeholder = Name
priority = Autoconnect priority
metered = Metered
metered-auto = Automatic
metered-yes = Yes
metered-no = No
zone = Firewall zone
split-tunnel = Use only for its own network
dns-search = DNS search domains
dns-servers = DNS servers
advanced-data = Advanced (data)
key-placeholder = Key
value-placeholder = Value
add-field = Add field

# OpenConnect editor
oc-general = General
oc-cert-auth = Certificate Authentication
oc-protocol = VPN Protocol
oc-ca-cert = CA Certificate
oc-proxy = Proxy
oc-user-agent = User Agent
oc-reported-version = Reported Version
oc-reported-os = Reported OS
oc-os-default = Default
oc-csd-trojan = Allow Cisco Secure Desktop trojan
oc-csd-wrapper = CSD Wrapper Script
oc-machine-cert = Machine Certificate
oc-private-key = Private Key
oc-user-cert = User Certificate
oc-fsid = Use FSID for key passphrase
oc-prevent-invalid = Prevent user from manually accepting invalid certificates

# Secret storage flags (NM secret-flags values)
secret-store = Store password
secret-ask = Always ask
secret-none = Not required
save = Save

# WireGuard editor
wg-private-key = Private key
wg-address = Address
wg-mtu = MTU
wg-peer = Peer
wg-peer-n = Peer { $index }
wg-peer-key = Public key
wg-peer-endpoint = Endpoint
wg-allowed-ips = Allowed IPs
wg-preshared-key = Pre-shared key
wg-keepalive = Persistent keepalive (s)
wg-add-peer = Add peer
remove = Remove
