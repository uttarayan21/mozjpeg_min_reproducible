#include <stdio.h>
#include <windows.h>

#define HAVE_BOOLEAN
#include <jpeglib.h>

void jpeg_test() {
  struct jpeg_decompress_struct cinfo;
  size_t x = sizeof(struct jpeg_decompress_struct);
  printf("%d\n", x);
  jpeg_create_decompress(&cinfo);
  printf("Unreachable\n");
}
