#!/bin/sh
mkdir /musl
apk add git wget tar musl-dev linux-headers perl make
cd /root
wget https://github.com/openssl/openssl/archive/OpenSSL_1_1_1g.tar.gz
tar zxvf OpenSSL_1_1_1g.tar.gz
cd ./openssl-OpenSSL_1_1_1g
CC="gcc -fPIE -pie" ./Configure no-shared no-async --prefix=/musl --openssldir=/musl/ssl linux-x86_64
make depend
make -j$(nproc)
make install
cd /root
git clone https://github.com/keshihoriuchi/get-ssm-parameter
cd ./get-ssm-parameter
OPENSSL_STATIC=true OPENSSL_DIR=/musl cargo build --release
