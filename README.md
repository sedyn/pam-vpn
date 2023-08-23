# PAM VPN module

## Files

- test.c
    - sample code that calls the `pam-vpn` module
- bootstrap.sh
    - build `pam-vpn` module and copy to `/lib/x86_64-linux-gnu/security`
- scripts/e2e
    - Dockerfile
        - test openVPN server enviroment with test user (testuser:testpass)
    - get-keys.sh
        - copy cert files from test openVPN server

## Test
### Environment
- Debian 12
- Docker

### Commands
```bash
cd scripts/e2e
# build docker image
docker build . -t vpn

# run test openVPN server
./run.sh

# get keys and generate test.conf file for testing
./get-keys.sh

# back to root folder
cd ../..

# build pam-vpn module and move to security folder
./bootstrop.sh

# build PAM test code
make

# run
sudo ./test
```

Inspired by [pam-rs](https://github.com/anowell/pam-rs)
