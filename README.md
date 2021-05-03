# CodeBench Agent

## Build

```sh
go build
```

## Run

```
./agent
```

## Demo

```
» curl 192.168.127.18:8080/exec -X POST --data '{"command":"uname -a"}'
{"command":"uname -a","stderr":"","stdout":"Linux localhost 4.14.55-84.37.amzn2.x86_64 #1 SMP Wed Jul 25 18:47:15 UTC 2018 x86_64 Linux\n"}
```
