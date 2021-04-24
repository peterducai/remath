#!/bin/bash

# ECDSA recommendation key ≥ secp384r1
# List ECDSA the supported curves (openssl ecparam -list_curves)
openssl req -new -x509 -nodes -newkey ec:<(openssl ecparam -name secp384r1) -keyout cert.key -out cert.crt -days 3650 -subj "/C=CZ/ST=Oregon/L=Portland/O=CompanyTest/OU=Org/CN=www.example.com"
# openssl req -x509 -nodes -newkey ec:<(openssl ecparam -name secp384r1) -keyout server.ecdsa.key -out server.ecdsa.crt -days 3650
# -pkeyopt ec_paramgen_curve:… / ec:<(openssl ecparam -name …) / -newkey ec:…
ln -sf server.ecdsa.key server.key
ln -sf server.ecdsa.crt server.crt

# RSA recommendation key ≥ 2048-bit
openssl req -x509 -nodes -newkey rsa:2048 -keyout server.rsa.key -out server.rsa.crt -days 3650 -subj "/C=CZ/ST=Oregon/L=Portland/O=CompanyTest/OU=Org/CN=www.example.com"
ln -sf server.rsa.key server.key
ln -sf server.rsa.crt server.crt