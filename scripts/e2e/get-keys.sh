#!/bin/bash

ID=$(docker ps | sed -n 2p | cut -d " " -f1)

rm -rf keys
mkdir keys
docker cp $ID:/etc/openvpn/server keys

cat <<EOF >> test.conf
client
proto udp
remote localhost 8081
dev tun
nobind
key-direction 1
remote-cert-tls server
cipher AES-256-CBC
auth-user-pass
auth-nocache
<ca>
$(<keys/server/ca.crt)
</ca>
<tls-auth>
$(<keys/server/ta.key)
</tls-auth>
EOF
