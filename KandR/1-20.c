#include <stdio.h>
#include <stdlib.h>

#define TABSTOP 4

// Write a program `detab` that replaces tab stops in the input 
// with the proper number of blanks to space to the next tab stop. Assume a fixed set of tab stops,
// say every *n* columns.
void insert(char *s, int idx, char toInsert, int times);
void detab(char *line);

int main(int argc, char **argv)
{
    char *line = NULL;
    size_t size = 0;
    while(getline(&line, &size, stdin) != -1)
    {
        detab(line);
        printf("%s", line);
        free(line);
        line = NULL;
        size = 0;
    }
    return 0; 
}


void detab(char *line)
{
    char c;
    int i = 0;
    while((c = line[i++]))
    {
        if(c == '\t')
        {
            insert(line, i, ' ', TABSTOP - (i % TABSTOP));
            detab(&(line[TABSTOP - (i % TABSTOP)]));
        }
    }
    return;
}

void insert(char *s, int idx, char toInsert, int times)
{
    // Implement this function. It should insert toInsert into s at idx times times by shifting everything
    // and then recursing.
}
