app-title = COSMIC VPN 메뉴

# Popup states
no-vpn-connections = VPN 연결 없음
load-error = VPN 연결을 불러올 수 없습니다: { $error }

# Connection status
vpn-status-connected = 연결됨
vpn-status-connecting = 연결 중…
vpn-status-disconnecting = 연결 해제 중…
vpn-status-disconnected = 연결 끊김
vpn-status-unknown = 알 수 없음

# Connection detail panel
detail-type = 유형
detail-gateway = 게이트웨이
detail-ipv4 = IPv4
detail-ipv6 = IPv6
dns-servers = DNS 서버
detail-duration = 연결 시간
autoconnect = 자동으로 연결

# Forget / delete
forget = 삭제
confirm-forget = 이 연결을 삭제하시겠습니까?
delete = 삭제
cancel = 취소

# Secret prompt
secret-prompt = { $name }의 자격 증명을 입력하세요
connect = 연결

# Import
import = 가져오기…

# Create / edit
add-vpn = VPN 추가
edit = 편집…
select-vpn-type = VPN 유형 선택
new-vpn = 새 { $kind } VPN
edit-vpn = { $kind } VPN 편집
name-placeholder = 이름
advanced-data = 고급 (데이터)
key-placeholder = 키
value-placeholder = 값
add-field = 필드 추가
save = 저장

# WireGuard editor
wg-private-key = 개인 키
wg-address = 주소
wg-peer = 피어
wg-peer-key = 공개 키
wg-peer-endpoint = 엔드포인트
wg-allowed-ips = 허용된 IP

# WireGuard / secrets (added)
wg-mtu = MTU
wg-peer-n = 피어 { $index }
wg-preshared-key = 사전 공유 키
wg-keepalive = 지속적 연결 유지 (초)
wg-add-peer = 피어 추가
remove = 제거
secret-store = 비밀번호 저장
secret-ask = 항상 묻기
secret-none = 필요 없음

# Sync (backfilled)
notify-connected = VPN 연결됨
notify-disconnected = VPN 연결 끊김
notify-failed = VPN 연결 실패
reason-ip-config = 잘못된 IP 구성
reason-timeout = 연결 시간이 초과됨
reason-service-failed = VPN 서비스를 시작하지 못함
reason-no-secrets = 비밀번호가 제공되지 않음
reason-login-failed = 인증 실패
detail-dns = DNS
detail-received = 받음
detail-sent = 보냄
detail-last-used = 마지막 사용
duplicate = 복제
priority = 자동 연결 우선순위
metered = 종량제
metered-auto = 자동
metered-yes = 예
metered-no = 아니요
zone = 방화벽 영역
split-tunnel = 자체 네트워크에만 사용
dns-search = DNS 검색 도메인

# OpenConnect / OpenVPN / vpnc (added)
oc-general = 일반
oc-cert-auth = 인증서 인증
oc-protocol = VPN 프로토콜
oc-ca-cert = CA 인증서
oc-proxy = 프록시
oc-user-agent = 사용자 에이전트
oc-reported-version = 보고된 버전
oc-reported-os = 보고된 OS
oc-os-default = 기본값
oc-csd-trojan = Cisco Secure Desktop 트로이 목마 허용
oc-csd-wrapper = CSD 래퍼 스크립트
oc-machine-cert = 컴퓨터 인증서
oc-private-key = 개인 키
oc-user-cert = 사용자 인증서
oc-fsid = 키 암호에 FSID 사용
oc-prevent-invalid = 사용자가 잘못된 인증서를 수동으로 수락하지 못하도록 방지
ovpn-conn-type = 연결 유형
ovpn-ct-tls = 인증서 (TLS)
ovpn-ct-password = 비밀번호
ovpn-ct-password-tls = 인증서와 비밀번호 (TLS)
ovpn-ct-static-key = 정적 키
ovpn-username = 사용자 이름
ovpn-static-key = 정적 키
ovpn-key-dir = 키 방향
ovpn-remote-ip = 원격 IP 주소
ovpn-local-ip = 로컬 IP 주소
ovpn-advanced = 고급
ovpn-port = 게이트웨이 포트
ovpn-tcp = TCP 연결 사용
ovpn-cipher = 암호
ovpn-auth = HMAC 인증
vpnc-group = 그룹 이름
vpnc-domain = 도메인
vpnc-nat = NAT 통과
vpnc-pfs = Perfect Forward Secrecy
vpnc-dh = DH 그룹
vpnc-vendor = 공급업체
vpnc-app-version = 애플리케이션 버전
vpnc-single-des = 취약한 single DES 암호화 사용
