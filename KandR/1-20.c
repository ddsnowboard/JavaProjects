#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define TABSTOP 4

// Write a program `detab` that replaces tab stops in the input 
// with the proper number of blanks to space to the next tab stop. Assume a fixed set of tab stops,
// say every *n* columns.
void insert(char *s, int idx, char toInsert, int times);
void unshift(char *s, int idx);
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
    char c = line[0];
    int i = 0;
    do
    {
        if(c == '\t')
        {
            line = (char *) realloc((void *) line, strlen(line) + 5);
            insert(line, i, ' ', TABSTOP - (i % TABSTOP));
            detab(line);
        }
    } while((c = line[++i]));
    return;
}

void insert(char *s, int idx, char toInsert, int times)
{
    // Implement this function. It should insert toInsert into s at idx times times by shifting everything
    // and then recursing.
    if(times == 0)
        return;
    char last = s[idx];
    int index = idx;
    s[idx] = toInsert;
    ++idx;
    char temp;
    do
    {
        temp = s[idx];
        s[idx] = last;
        last = temp;
    } while(s[idx++] != '\0');
    insert(s, index, toInsert, --times);
}

void unshift(char *s, int idx)
{
    // This should remove a character and unshift everything.
}

