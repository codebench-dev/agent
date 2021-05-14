#!/bin/bash

set -xe

dd if=/dev/zero of=rootfs.ext4 bs=1M count=200
mkfs.ext4 rootfs.ext4
mkdir -p /tmp/my-rootfs
mount rootfs.ext4 /tmp/my-rootfs

docker run -i --rm \
    -v /tmp/my-rootfs:/my-rootfs \
    -v "$(pwd)/target/x86_64-unknown-linux-musl/release/agent:/usr/local/bin/agent" \
    -v "$(pwd)/openrc-service.sh:/etc/init.d/agent" \
    alpine sh < setup-alpine.sh

umount /tmp/my-rootfs

# rootfs available under `rootfs.ext4`
