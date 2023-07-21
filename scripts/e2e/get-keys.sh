#!/bin/bash

ID=$(docker ps | sed -n 2p | cut -d " " -f1)

rm -rf keys
mkdir keys
docker cp $ID:/etc/openvpn/server keys
