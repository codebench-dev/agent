#!/bin/bash

set -xe

git clone https://github.com/torvalds/linux.git linux
cd linux
git checkout v4.20
wget https://raw.githubusercontent.com/firecracker-microvm/firecracker/main/resources/microvm-kernel-x86_64.config -O .config
make -j16 vmlinux

# uncompressed kernel image available under ./vmlinux
