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
   int kvmfd = open("/dev/rkvm", O_RDWR);
   if (kvmfd < 0) {
      perror("open /dev/rkvm");
      return -1;
   }
   int api = ioctl(kvmfd, RKVM_GET_API_VERSION, NULL);
   printf("rkvm api version: %d\n", api);

   int vmfd = ioctl(kvmfd, RKVM_CREATE_VM, NULL);
   printf("vmfd: %d\n", vmfd);
   return 0;
}

