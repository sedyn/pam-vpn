FROM debian

WORKDIR /root/

RUN apt update \
    && apt install -y openvpn easy-rsa iptables \
    && rm -rf /var/lib/apt/lists/*

COPY ./install.sh ./install.sh

RUN chmod +x ./install.sh && bash ./install.sh

# Add user to test openvpn-plugin-auth-pam.so
RUN useradd -ms /bin/bash testuser && echo "testuser:testpass" | chpasswd

COPY ./server.conf ./server.conf

CMD [ "openvpn", "--config", "server.conf" ]
