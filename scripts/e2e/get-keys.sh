#!/bin/bash

ID=$(docker ps | sed -n 2p | cut -d " " -f1)

rm -rf keys
mkdir keys
docker cp $ID:/etc/openvpn/server keys

cat <<EOF >> test.conf
client
proto udp
remote localhost 8081
prot 1194
dev tun
nobind
key-direction 1
<ca>
$(<keys/server/ca.crt)
</ca>
EOF
