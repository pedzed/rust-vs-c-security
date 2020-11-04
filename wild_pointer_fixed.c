#include <stdlib.h>
#include <stdio.h>

int main()
{
    int *ptr = malloc(sizeof *ptr);
    *ptr = 123;
    printf("ptr = %p (%d)\n", ptr, *ptr);

    char *ptr2 = malloc(sizeof *ptr2);
    *ptr2 = 'A';
    printf("ptr = %p (%c)\n", ptr2, *ptr2);
}
