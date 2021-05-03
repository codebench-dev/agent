#!/sbin/openrc-run

name=$RC_SVCNAME
description="CodeBench agent"
supervisor="supervise-daemon"
command="/usr/local/bin/agent"

depend() {
	after net
}
