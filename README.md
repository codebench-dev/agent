# CodeBench Agent

To build the filesystem of the agent VM, see details in https://github.com/codebench-esgi/worker.

## Build

```
cargo build
```

With musl (for Alpine Linux):

```sh
rustup target add x86_64-unknown-linux-musl
cargo build --target x86_64-unknown-linux-musl
```

## Run

```sh
./agent
```

## Demo

## Healthcheck

```sh
» curl -i 127.0.0.1:8080/health
HTTP/1.1 200 OK
Content-Type: text/plain; charset=UTF-8
Date: Tue, 11 May 2021 20:03:48 GMT
Content-Length: 2

OK
```

### Run command

```sh
» curl 127.0.0.1:8080/exec -X POST -H 'Content-Type: application/json' --data '{"command":"uname -a"}'
{"command":"uname -a","stdout":"Linux devenv 4.15.0-140-generic #144-Ubuntu SMP Fri Mar 19 14:12:35 UTC 2021 x86_64 x86_64 x86_64 GNU/Linux\n","stderr":""}
```

### Compile and run standalone C code

```sh
» curl -i 127.0.0.1:8080/run/c -X POST -H 'Content-Type: application/json' --data '{"code":"#include <stdio.h>\r\nint main() {\r\n   \/\/ printf() displays the string inside quotation\r\n   printf(\"Hello, World!\");\r\n   return 0;\r\n}"}'
HTTP/1.1 200 OK
Content-Type: application/json; charset=UTF-8
Date: Tue, 11 May 2021 20:03:14 GMT
Content-Length: 59

{"message":"Success","stdout":"Hello, World!","stderr":""}
```
