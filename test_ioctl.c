#include <stdio.h>
#include <sys/ioctl.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>


/* Define ioctl commands */
#define RKVM_GET_API_VERSION 0
#define RKVM_CREATE_VM 1


int main(void)
{
    int rkvmfd = open("/dev/rkvm", O_RDWR);
    if (rkvmfd < 0) {
       perror("open /dev/rkvm");
       return -1;
    }
    int api = ioctl(rkvmfd, RKVM_GET_API_VERSION, NULL);
    printf("rkvm api version: %d\n", api);

    int vmfd = ioctl(rkvmfd, RKVM_CREATE_VM, NULL);
    if (vmfd < 0) {
       perror("rkvm create vm");
       return -1;
    }

    int vm_fd_test = ioctl(vmfd, 1, NULL);
    printf("vm fd test: %d\n", vm_fd_test);
    return 0;
}

