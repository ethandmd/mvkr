References: 
+ [setting up env for writing linux kernel modules in rust](https://www.youtube.com/watch?v=tPs1uRqOnlk)
+ [mini linux](https://gist.github.com/chrisdone/02e165a0004be33734ac2334f215380e)

## Download
Get modified kernel: 
```
$ git clone git@github.com:ethandmd/linux.git
```

Get Busybox to build image:
`$ git clone https://github.com/mirror/busybox.git`

## Configure Linux:

Change to Linux directory:
`$ cd linux`
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

I've included a 'full' and a 'min' config for x86 you can use to compile the kernel (ymmv), or config yourself:

Here is the general recipe to compile a minimal linux with support for rust, enough to use on qemu + busybox, and use loadable modules:
```
$ make LLVM=1 allnoconfig qemu-busybox-min.config rust.config
```
then, `make menuconfig` and select `Enable loadable module support` from main menu.

## Configure Busybox:

Then, go to busybox dir
`$ cd ../busybox`
and run the following to use default config
`$ make defconfig`
before opting to statically compile so that we don't have to add libc to our image
`$ make menuconfig` -> select `settings` -> select `Build static binary (no shared libs)`
Finally, build and install busybox:
`$ make -j$(nproc)`.
`$ make install`.
Go to _install dir and create any missing mount points you'll want later. Then, copy the `inittab` from `busybox/examples` into 
your `_install` folder:
```
$ mkdir _install/etc
$ cp examples/inittab _install/etc 
```
Then remove the following lines from `_install/etc/inittab`, example diff between original (left) and modified (right):
```
   1 +-- 68 lines: # /etc/inittab init(8) configuration for BusyBox·························│+   1 +-- 68 lines: # /etc/inittab init(8) configuration for BusyBox························
   69 # login, but since we are bypassing login in this case, BusyBox lets you do            │   69 # login, but since we are bypassing login in this case, BusyBox lets you do
   70 # this yourself...                                                                     │   70 # this yourself...
   71 #                                                                                      │   71 #
   72 # Start an "askfirst" shell on the console (whatever that may be)                      │   72 # Start an "askfirst" shell on the console (whatever that may be)
   73 ::askfirst:-/bin/sh                                                                    │   73 ::askfirst:-/bin/sh
   74 # Start an "askfirst" shell on /dev/tty2-4                                             │   74 # Start an "askfirst" shell on /dev/tty2-4
   75 tty2::askfirst:-/bin/sh                                                                │      --------------------------------------------------------------------------------------
   76 tty3::askfirst:-/bin/sh                                                                │      --------------------------------------------------------------------------------------
   77 tty4::askfirst:-/bin/sh                                                                │      --------------------------------------------------------------------------------------
   78                                                                                        │      --------------------------------------------------------------------------------------
   79 # /sbin/getty invocations for selected ttys                                            │      --------------------------------------------------------------------------------------
   80 tty4::respawn:/sbin/getty 38400 tty5                                                   │      --------------------------------------------------------------------------------------
   81 tty5::respawn:/sbin/getty 38400 tty6                                                   │      --------------------------------------------------------------------------------------
   82                                                                                        │      --------------------------------------------------------------------------------------
   83 # Example of how to put a getty on a serial line (for a terminal)                      │   75 # Example of how to put a getty on a serial line (for a terminal)
   84 #::respawn:/sbin/getty -L ttyS0 9600 vt100                                             │   76 #::respawn:/sbin/getty -L ttyS0 9600 vt100
   85 #::respawn:/sbin/getty -L ttyS1 9600 vt100                                             │   77 #::respawn:/sbin/getty -L ttyS1 9600 vt100
   86 #                                                                                      │   78 #
   87 # Example how to put a getty on a modem line.                                          │   79 # Example how to put a getty on a modem line.
   88 #::respawn:/sbin/getty 57600 ttyS2                                                     │   80 #::respawn:/sbin/getty 57600 ttyS2
+  89 +--  8 lines: # Stuff to do when restarting the init process···························│+  81 +--  8 lines: # Stuff to do when restarting the init process·
```
(diff command: `nvim -d file1 file2`).

Then add the following init file in _install/init.d/rcS (create `init.d/` if it doesn't exit):
```
mkdir -p /proc
mkdir -p /sys
mount -t proc none /proc
mount -t sysfs none /sys
mount -t devtmpfs none /dev
```

## Build out of tree rust kernel module:
Return to top level directory (`mvkr/`) and:
```
$ cd rkvm/
$ make # Hardcoded LLVM=1 in Makefile
$ cp rkvm.ko ../busybox/_install # Copy kernel mod for packaging in busybox image. 
```

Now, generate and compress our busybox image:
```
$ cd ../busybox/_install
$ find . | cpio -H newc -o | gzip > ../ramdisk.img.gz
```

## Run
Finally, return to top level directory and run qemu with busybox image and our minimal linux kernel.
```
$ cd ../.. # Return to top level directory (aka mvkr/)
$ qemu-system-x86_64 -enable-kvm -kernel linux/arch/x86/boot/bzImage -initrd busybox/ramdisk.img.gz -nographic
```
### Load rkvm.ko:
Within qemu, load the kernel mod you copied into busybox image:
```
# insmod rkvm.ko # Module should be in root directory
# dmesg | tail -5 # Check dmesg (ok if tainted kernel after loading our custom module)
# ls -l /dev/rkvm # Check to see that misc device is registered!
[optional]
# rmmod rkvm.ko
```
