# TLS 1.3 Client Hello

Rust による TLS 1.3 Client Hello 実装。

コマンドラインのパース部分(`clap`)以外は std ライブラリのみを用いる。

# Usage (MacOS)

## install Openssl via Homebrew

```
$ brew install openssl
$ brew link openssl
$ echo 'export PATH="/opt/homebrew/opt/openssl@3/bin:$PATH"' >> ~/.zshrc
```

## create an RSA private key

```
$ openssl genpkey -out private.key 2048
```

## create a Certificate Signing Request (CSR)

```
$ openssl req -new -key private.key -out server.csr
```

## sing a CSR using `private.key`

```
$ openssl x509 -req -in server.csr -signkey private.key -out server.crt
```

## launch TLS 1.3 server

```
openssl s_server -tls1_3 -cert server.crt -key private.key
```

## client hello

```
$ cargo run -- --ipv4-address 127.0.0.1 --port 4433
```
