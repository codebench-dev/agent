#!/bin/bash

set -xe

dd if=/dev/zero of=rootfs.ext4 bs=1M count=50
mkfs.ext4 rootfs.ext4
mkdir -p /tmp/my-rootfs
mount rootfs.ext4 /tmp/my-rootfs
docker run -i --rm -v /tmp/my-rootfs:/my-rootfs alpine sh < setup-alpine.sh
cp agent /tmp/my-rootfs/usr/local/bin/agent
cp openrc-service.sh /tmp/my-rootfs/etc/init.d/agent
umount /tmp/my-rootfs

# rootfs available under `rootfs.ext4`
