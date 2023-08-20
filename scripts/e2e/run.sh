#!/bin/bash
docker run --rm -p 8081:1194/udp --device /dev/net/tun --privileged vpn
