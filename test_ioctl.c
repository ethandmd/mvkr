#include <stdio.h>
#include <sys/ioctl.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>


/* Define ioctl commands */
#define IOCTL_ONE 0x00000001
#define IOCTL_FOUR   0x00000004


int main(void)
{
   int fd = open("/dev/rkvm", O_RDWR);
   if (fd < 0) {
      perror("open /dev/rkvm");
      return -1;
   }
   ioctl(fd, IOCTL_ONE);
   return 0;
}

