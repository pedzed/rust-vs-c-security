#include <stdio.h>

int main()
{
    int *ptr;
    printf(
        "ptr=%p (%d)\n",
        (void *) ptr, *ptr
    );

    char *ptr2;
    printf(
        "ptr2=%p (%c)\n",
        (void *) ptr2, *ptr2
    );
}
