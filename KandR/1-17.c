#define MAXLINE 5000
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Write a program to print all input lines that are longer than 80 characters
char* getLine(FILE *f);

int main(int argc, char** argv)
{
    FILE *f = argc == 1 ? stdin : fopen(argv[1], "r");
    char *curr;
    while(strcmp(curr = getLine(f), "") != 0)
    {
        if(strlen(curr) > 80)
            printf("%s", curr);
        free(curr);
    }
    return 0;
}

char* getLine(FILE *f)
{
    char *line = malloc(MAXLINE);
    int idx = 0;
    char curr = fgetc(f);
    if(curr == EOF)
    {
        free(line);
        return "";
    }
    do
        line[idx++] = curr;
    while((curr = fgetc(f)) != EOF && curr != '\n');
    return line;
}
