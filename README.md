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
» curl 127.0.0.1:8080/exec -X POST -H 'Content-Type: application/json' --data '{"command":"uname -a"}'
{"command":"uname -a","stdout":"Linux devenv 4.15.0-140-generic #144-Ubuntu SMP Fri Mar 19 14:12:35 UTC 2021 x86_64 x86_64 x86_64 GNU/Linux\n","stderr":""}
```
