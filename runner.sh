#!/bin/bash

cd rkvm/
make
cp rkvm.ko ../busybox/_install/

cd ../
clang -static test_ioctl.c -o test_ioctl
cp test_ioctl busybox/_install/

cd busybox/_install/
find . | cpio -H newc -o | gzip > ../ramdisk.img.gz

cd ../../
qemu-system-x86_64 -enable-kvm -kernel linux/arch/x86/boot/bzImage -initrd busybox/ramdisk.img.gz -nographic
