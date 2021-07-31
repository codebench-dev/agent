# CodeBench Agent

To build the filesystem of the agent VM, see details in [codebench-dev/worker](https://github.com/codebench-dev/worker)

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
Content-Type: text/plain; charset=UTF-8
Date: Sat, 31 Jul 2021 20:02:45 GMT
Content-Length: 2
```

### Compile and run standalone code

### C

```sh
» curl -i localhost:8080/run -X POST --data '{"code":"#include <stdio.h>\r\nint main() {\r\n   printf(\"Hello, C!\");\r\n   return 0;\r\n}","id":"123","variant":"gcc","language":"c"}' -H 'Content-Type: application/json'
HTTP/1.1 200 OK
Content-Type: application/json; charset=UTF-8
Date: Sat, 31 Jul 2021 20:04:04 GMT
Content-Length: 104

{"message":"Success","error":"","stdout":"Hello, C!","stderr":"","exec_duration":1843,"mem_usage":9432}
```

### C++

```
» curl -i -H 'Content-Type: application/json' -X POST http://localhost:8080/run --data '{"id":"1235","code":"#include <iostream>\n\nint main() {\n    std::cout << \"Hello, C++!\";\n    return 0;\n}\n","language":"cpp", "variant":"cpp"}'
HTTP/1.1 200 OK
Content-Type: application/json; charset=UTF-8
Date: Sat, 31 Jul 2021 20:04:50 GMT
Content-Length: 106

{"message":"Success","error":"","stdout":"Hello, C++!","stderr":"","exec_duration":3899,"mem_usage":9972}
```

### Python

```
» curl -i -H 'Content-Type: application/json' -X POST http://localhost:8080/run --data '{"id":"1234","code":"print(\"Hello, Python!\")","language":"python", "variant":"cpython3"}'
HTTP/1.1 200 OK
Content-Type: application/json; charset=UTF-8
Date: Sat, 31 Jul 2021 20:06:31 GMT
Content-Length: 113

{"message":"Success","error":"","stdout":"Hello, Python!\n","stderr":"","exec_duration":20958,"mem_usage":10620}
```

### Go

```
» curl -i localhost:8080/run -X POST --data '{"code":"package main\r\n\r\nimport \"fmt\"\r\n\r\nfunc main() {\r\n    fmt.Println(\"Hello, Go!\")\r\n}","id":"123","variant":"go","language":"go"}' -H 'Content-Type: application/json'
HTTP/1.1 200 OK
Content-Type: application/json; charset=UTF-8
Date: Sat, 31 Jul 2021 20:07:14 GMT
Content-Length: 108

{"message":"Success","error":"","stdout":"Hello, Go!\n","stderr":"","exec_duration":4706,"mem_usage":10620}
```
