#!/bin/sh

cargo build

sudo mv ./target/debug/libpam_vpn.so /lib/x86_64-linux-gnu/security
sudo chmod 755 /lib/x86_64-linux-gnu/security/libpamvpn.so
