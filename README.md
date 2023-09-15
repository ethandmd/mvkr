References: 
+ [setting up env for writing linux kernel modules in rust](https://www.youtube.com/watch?v=tPs1uRqOnlk)
+ [mini linux](https://gist.github.com/chrisdone/02e165a0004be33734ac2334f215380e)

## Download
Get modified kernel: 
```
$ git clone git@github.com:ethandmd/rkvm.git
```

Get Busybox to build image:
`$ git clone https://github.com/mirror/busybox.git`

## Configure Linux:

Change to Linux directory:
`$ cd rkvm`
and check:
`$ make LLVM=1 rustavailable`
reference `Documentation/rust/quick-start.rst`
```
$ make LLVM=1 rustavailable
Rust is available!
```
Also:
```
$ make rust-analyzer
```

Lastly, you'll need all the usual suspects like `clang`,`llvm`, `flex`, `bison`, ...

DEMO CONFIG (ymmv) included, or config yourself:

Use `make allnoconfig` or `defconfig`, etc... to suit your needs. However, ensure Rust support is enabled in addition to most of the following from the miny linux reference:
```
64-bit kernel ---> yes
General setup ---> Initial RAM filesystem and RAM disk (initramfs/initrd) support ---> yes
General setup ---> Configure standard kernel features ---> Enable support for printk ---> yes
Executable file formats / Emulations ---> Kernel support for ELF binaries ---> yes
Executable file formats / Emulations ---> Kernel support for scripts starting with #! ---> yes
Device Drivers ---> Generic Driver Options ---> Maintain a devtmpfs filesystem to mount at /dev ---> yes
Device Drivers ---> Generic Driver Options ---> Automount devtmpfs at /dev, after the kernel mounted the rootfs ---> yes
Device Drivers ---> Character devices ---> Enable TTY ---> yes
Device Drivers ---> Character devices ---> Serial drivers ---> 8250/16550 and compatible serial support ---> yes
Device Drivers ---> Character devices ---> Serial drivers ---> Console on 8250/16550 and compatible serial port ---> yes
File systems ---> Pseudo filesystems ---> /proc file system support ---> yes
File systems ---> Pseudo filesystems ---> sysfs file system support ---> yes
```

## Configure Busybox:

Then, go to busybox dir
`$ cd ../busybox`
and run the following to use default config
`$ make defconfig`
before opting to statically compile so that we don't have to add libc to our image
`$ make menuconfig` -> select `settings` -> select `build static ...`
Finally, build and install busybox:
`$ make -j$(nproc)`.
`$ make install`.
Go to _install dir and:
`$ mkdir bin sbin etc proc sys usr/bin usr/sbin`
In case some, like `etc`, don't already exist, then add the following init file in _install/init:
```
#!/bin/sh

mount -t proc none /proc
mount -t sysfs none /sys

cat <<!


Boot took $(cut -d' ' -f1 /proc/uptime) seconds

           (       
 ( (   (   )\  (   
 )\)\  )\:((_) )\  
(_((_)((_)_ |__( ) 
| '  \\ V / / / '_|
|_|_|_|\_/|_\_\_|  

Welcome to mvkr


!
exec /bin/sh
```
Make the init file executable:
`$ chmod+x init`

Now, grab our image to put our kernel:
`$ find . -print0 | cpio --null -ov --format=newc | gzip -9 > ../ramdisk.img.gz`.

## Run:
```
$ cd ..
$ qemu-system-x86_64 -enable-kvm -kernel rkvm/arch/x86/boot/bzImage -initrd busybox/ramdisk.img.gz -nographic -append "console=ttyS0"
```

Instructions on loading rkvm module to follow...Initial plan is to load it in `rust/samples/rkvm.rs`

### Rough expectation:
```
[    0.663473] Freeing unused kernel image (initmem) memory: 2648K
[    0.666263] Write protecting the kernel read-only data: 26624k
[    0.666791] Freeing unused kernel image (rodata/data gap) memory: 1296K
[    0.698919] x86/mm: Checked W+X mappings: passed, no W+X pages found.
[    0.699245] x86/mm: Checking user space page tables
[    0.729200] x86/mm: Checked W+X mappings: passed, no W+X pages found.
[    0.729516] Run /init as init process
[    0.730157] mount (47) used greatest stack depth: 14456 bytes left


Boot took 0.68 seconds

           (       
 ( (   (   )\  (   
 )\)\  )\:((_) )\  
(_((_)((_)_ |__( ) 
| '  \ V / / / '_|
|_|_|_|\_/|_\_\_|  

Welcome to mvkr


/bin/sh: can't access tty; job control turned off
~ # [    1.273909] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input3

~ #
```
