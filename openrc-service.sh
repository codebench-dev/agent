#!/sbin/openrc-run

name=$RC_SVCNAME
description="CodeBench agent"
supervisor="supervise-daemon"
command="/usr/local/bin/agent"
pidfile="/run/agent.pid"
command_user="codebench:codebench"

depend() {
	after net
}
