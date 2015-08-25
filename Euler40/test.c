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
    return dest - 2;
}
int main(int argc, char** argv)
{
    char* a = malloc(10);
    char* b = malloc(10);
    char* c = "apples";
    strcpy(a, ".");
    strcpy(b, "2");
    char* i = mystrcat(a, b);
    printf("%s\n", a);
    mystrcat(i, c);
    printf("%s\n", a);
}
