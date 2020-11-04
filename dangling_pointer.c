#include <stdio.h>

int *dangling_address()
{
    int a = 10;
    return &a;
}

int main()
{
    int *ptr;
    ptr = dangling_address();

    printf(
        "ptr = %p (%d)\n",
        (void *) ptr, *ptr
    );
}
