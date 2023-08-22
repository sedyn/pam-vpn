# PAM VPN module

## Files

- test.c
    - sample code that calls the `pam-vpn` module
- bootstrap.sh
    - build `pam-vpn` module and copy to `/lib/x86_64-linux-gnu/security`
- scripts/e2e
    - Dockerfile
        - test openVPN server enviroment
    - get-keys.sh
        - copy cert files from test openVPN server

Inspired by [pam-rs](https://github.com/anowell/pam-rs)
