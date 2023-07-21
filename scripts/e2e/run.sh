#!/bin/bash
docker run --rm -p 8081:1194 --device /dev/net/tun --cap-add=NET_ADMIN vpn
