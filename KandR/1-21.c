#define MAXLEN 80
#include <stdio.h>

// Wirte a program to "fold" long input lines into two or more shorter lines after the last non-blank
// character that occurs before the n-th column of input. Make sure your program does something intelligent with very long lines, 
// and if there are no blanks or tabs before the specified column.

void fold(char **line);
void insert(char *s, int idx, char toInsert, int times);

int main(int argc, char **argv)
{
    char *line = NULL;
    size_t size = 0;
    while(getline(&line, &size, stdin) != -1)
    {
        fold(line);
        printf("%s", line);
        line = NULL;
        size = 0;
    }
}

void fold(char **line)
{

}
