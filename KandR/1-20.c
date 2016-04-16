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
char *embiggen(char *s, char toCount, int factor);

int main(int argc, char **argv)
{
    char *line = NULL;
    size_t size = 0;
    while(getline(&line, &size, stdin) != -1)
    {
        line = embiggen(line, '\t', TABSTOP);
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
    // If you don't have enough memory allocated to hold the end result, you're screwwwwwwwwwed
    char c = line[0];
    int i = 0;
    do
    {
        if(c == '\t')
        {
            unshift(line, i);
            insert(line, i, ' ', TABSTOP - (i % TABSTOP));
            detab(line);
            return;
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
    do
    {
        s[idx] = s[idx + 1];
    } while(s[++idx]);
}

char *embiggen(char *s, char toCount, int factor)
{
    // This function allocates enough memory to hold a maximum factor characters replacing each toCount
    char *walker = s;
    int count = 0;
    while(*walker++)
        if(*walker == toCount)
            count++;
    return (char *) realloc((void *)s, strlen(s) + factor * count);
}
