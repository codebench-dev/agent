# CodeBench Agent

To build the filesystem of the agent VM, see details in https://github.com/codebench-esgi/worker.

## Build

```sh
go build
```

Static (for Alpine Linux):

```sh
go build -tags netgo -ldflags '-extldflags "-static"'
```

## Run

```sh
./agent
```

## Demo

### Healthcheck

```sh
» curl -i 127.0.0.1:8080/health
HTTP/1.1 200 OK
content-length: 0
date: Fri, 14 May 2021 22:36:04 GMT
```

### Run command

```sh
» curl -i 127.0.0.1:8080/exec -X POST --data '{"command":"uname -a"}' -H 'Content-Type: application/json'
HTTP/1.1 200 OK
content-length: 155
content-type: application/json
date: Fri, 14 May 2021 22:37:00 GMT

{"command":"uname -a","stdout":"Linux devenv 4.15.0-142-generic #146-Ubuntu SMP Tue Apr 13 01:11:19 UTC 2021 x86_64 x86_64 x86_64 GNU/Linux\n","stderr":""}
```

### Compile and run standalone C code

```sh
» curl -i 127.0.0.1:8080/run/c -X POST -H 'Content-Type: application/json' --data '{"id": "1234", "code":"#include <stdio.h>\r\nint main() {\r\n   \/\/ printf() displays the string inside quotation\r\n   printf(\"Hello, World!\");\r\n   return 0;\r\n}"}'
HTTP/1.1 200 OK
content-length: 57
content-type: application/json
date: Fri, 14 May 2021 22:36:41 GMT

{"message":"stonks","stdout":"Hello, World!","stderr":""}
```
