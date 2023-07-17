FROM debian

WORKDIR /root/

RUN apt update \
    && apt install -y openvpn easy-rsa iptables \
    && rm -rf /var/lib/apt/lists/*

COPY ./install.sh ./install.sh

RUN chmod +x ./install.sh && bash ./install.sh

COPY ./server.conf ./server.conf

CMD [ "openvpn", "--config", "server.conf" ]
