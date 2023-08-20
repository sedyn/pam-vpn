#!/bin/sh

cargo build

sudo cp scripts/e2e/test.conf /etc/pam.d/client.conf
sudo mv ./target/debug/libpam_vpn.so /lib/x86_64-linux-gnu/security
sudo chmod 755 /lib/x86_64-linux-gnu/security/libpam_vpn.so
