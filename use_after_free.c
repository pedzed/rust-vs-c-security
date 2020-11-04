#include <stdlib.h>
#include <stdio.h>
#include <string.h>

int main()
{
    char *hello = malloc(10 * sizeof(*hello));

    strcpy(hello, "Hello!");
    printf("hello=%s\n", hello);

    free(hello);

    // NOTE: use-after-free...
    printf("hello=%s\n", hello);
}
