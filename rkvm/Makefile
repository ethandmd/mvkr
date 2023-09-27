obj-m+=rkvm.o

KDIR ?= ../linux/

all:
	make LLVM=1 -C $(KDIR) M=$(PWD) modules

clean:
	make -C $(KDIR) M=$(PWD) clean
