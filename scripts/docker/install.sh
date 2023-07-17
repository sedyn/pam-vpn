#!/bin/bash

mkdir easy-rsa
ln -s /usr/share/easy-rsa/* easy-rsa
chmod 777 easy-rsa

cd easy-rsa

./easyrsa init-pki
echo | ./easyrsa build-ca nopass
echo | ./easyrsa gen-req server nopass
cp ./pki/private/server.key /etc/openvpn/server/
echo "yes" | ./easyrsa sign-req server server
cp ./pki/issued/server.crt /etc/openvpn/server/
cp ./pki/ca.crt /etc/openvpn/server/
./easyrsa gen-dh
openvpn --genkey secret ta.key
cp ./ta.key /etc/openvpn/server/
cp ./pki/dh.pem /etc/openvpn/server/
