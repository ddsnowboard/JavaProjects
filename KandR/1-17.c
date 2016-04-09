#define MAXLINE 5000
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Write a program to print all input lines that are longer than 80 characters

int main(int argc, char** argv)
{
    FILE *f = argc == 1 ? stdin : fopen(argv[1], "r");
    size_t size = 0;
    ssize_t ret;
    char *curr;
    curr = NULL;
    getline(&curr, 0, f);
    while((ret = getline(&curr, &size, f)) != -1)
    {
        if(strlen(curr) > 80)
            printf("%s", curr);
        free(curr);
        curr = NULL;
    }
    return 0;
}
