#!/bin/bash

cd rkvm/
make
cp rkvm.ko ../busybox/_install/

cd ../busybox/_install/
find . | cpio -H newc -o | gzip > ../ramdisk.img.gz

cd ../..
cp min-sample-x86.config linux/.config
cd linux/
make LLVM=1 -j$(nproc)

cd ../
qemu-system-x86_64 -enable-kvm -kernel linux/arch/x86/boot/bzImage -initrd busybox/ramdisk.img.gz -nographic
