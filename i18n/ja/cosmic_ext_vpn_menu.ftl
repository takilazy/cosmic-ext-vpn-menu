app-title = COSMIC VPN メニュー

# Popup states
no-vpn-connections = VPN 接続がありません
load-error = VPN 接続を読み込めませんでした: { $error }

# Connection status
vpn-status-connected = 接続済み
vpn-status-connecting = 接続中…
vpn-status-disconnecting = 切断中…
vpn-status-disconnected = 切断済み
vpn-status-unknown = 不明

# Connection detail panel
detail-type = 種類
detail-gateway = ゲートウェイ
detail-ipv4 = IPv4
detail-ipv6 = IPv6
dns-servers = DNS サーバー
detail-duration = 接続時間
autoconnect = 自動的に接続する

# Forget / delete
forget = 削除
confirm-forget = この接続を削除しますか？
delete = 削除
cancel = キャンセル

# Secret prompt
secret-prompt = { $name } の認証情報を入力してください
connect = 接続

# Import
import = インポート…

# Create / edit
add-vpn = VPN を追加
edit = 編集…
select-vpn-type = VPN の種類を選択
new-vpn = 新しい { $kind } VPN
edit-vpn = { $kind } VPN を編集
name-placeholder = 名前
advanced-data = 詳細設定 (データ)
key-placeholder = キー
value-placeholder = 値
add-field = フィールドを追加
save = 保存

# WireGuard editor
wg-private-key = 秘密鍵
wg-address = アドレス
wg-peer = ピア
wg-peer-key = 公開鍵
wg-peer-endpoint = エンドポイント
wg-allowed-ips = 許可された IP

# WireGuard / secrets (added)
wg-mtu = MTU
wg-peer-n = ピア { $index }
wg-preshared-key = 事前共有鍵
wg-keepalive = 持続的キープアライブ (秒)
wg-add-peer = ピアを追加
remove = 削除
secret-store = パスワードを保存
secret-ask = 毎回確認する
secret-none = 不要

# Sync (backfilled)
notify-connected = VPN が接続されました
notify-disconnected = VPN が切断されました
notify-failed = VPN の接続に失敗しました
reason-ip-config = 無効な IP 構成
reason-timeout = 接続がタイムアウトしました
reason-service-failed = VPN サービスの起動に失敗しました
reason-no-secrets = パスワードが指定されていません
reason-login-failed = 認証に失敗しました
detail-dns = DNS
detail-received = 受信
detail-sent = 送信
detail-last-used = 最終使用
duplicate = 複製
priority = 自動接続の優先度
metered = 従量制
metered-auto = 自動
metered-yes = はい
metered-no = いいえ
zone = ファイアウォールゾーン
split-tunnel = 自身のネットワークにのみ使用する
dns-search = DNS 検索ドメイン

# OpenConnect / OpenVPN / vpnc (added)
oc-general = 全般
oc-cert-auth = 証明書認証
oc-protocol = VPN プロトコル
oc-ca-cert = CA 証明書
oc-proxy = プロキシ
oc-user-agent = ユーザーエージェント
oc-reported-version = 報告バージョン
oc-reported-os = 報告 OS
oc-os-default = デフォルト
oc-csd-trojan = Cisco Secure Desktop トロイの木馬を許可
oc-csd-wrapper = CSD ラッパースクリプト
oc-machine-cert = マシン証明書
oc-private-key = 秘密鍵
oc-user-cert = ユーザー証明書
oc-fsid = 鍵のパスフレーズに FSID を使用
oc-prevent-invalid = 無効な証明書をユーザーが手動で受け入れられないようにする
ovpn-conn-type = 接続タイプ
ovpn-ct-tls = 証明書 (TLS)
ovpn-ct-password = パスワード
ovpn-ct-password-tls = 証明書付きパスワード (TLS)
ovpn-ct-static-key = 静的鍵
ovpn-username = ユーザー名
ovpn-static-key = 静的鍵
ovpn-key-dir = 鍵の方向
ovpn-remote-ip = リモート IP アドレス
ovpn-local-ip = ローカル IP アドレス
ovpn-advanced = 詳細
ovpn-port = ゲートウェイポート
ovpn-tcp = TCP 接続を使用
ovpn-cipher = 暗号
ovpn-auth = HMAC 認証
vpnc-group = グループ名
vpnc-domain = ドメイン
vpnc-nat = NAT トラバーサル
vpnc-pfs = Perfect Forward Secrecy
vpnc-dh = DH グループ
vpnc-vendor = ベンダー
vpnc-app-version = アプリケーションバージョン
vpnc-single-des = 脆弱な single DES 暗号化を有効にする
