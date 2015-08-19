#include <stdlib.h>
#include <stdio.h>
#include <string.h>
char* mystrcat(char* dest, char* src)
{
    while(*++dest)
    {
        printf("stepping through %c\n", *dest);
    }
    while(*dest++ = *src++)
    {
        printf("copying %c\n", *(dest - 1));
    }
    printf("is dest %s?\n", dest - 5);
    return --dest;
}
int main(int argc, char** argv)
{
    char* a = malloc(10);
    char* b = malloc(10);
    strcpy(a, "test");
    strcpy(b, "bed");
    mystrcat(a, b);
    printf("%s\n", a);
}
