#include <stdio.h>
#include <string.h>

// Write a function `reverse(s)` that reverses the character
// string `s`. Use it to write  program that reverses its input 
// a line at a time.

void reverse(char *s);
inline void swap(char *s, int a, int b);
double ceil(double l);
char *strip(char *s);

int main(int argc, char **argv)
{
    char *line = NULL;
    size_t size = NULL;
    while(getline(&line, &size, stdin) != -1)
    {
        reverse(line);
        printf("%s\n", line);
    }
    return 0;
}

void reverse(char *s)
{
    int i = 0;
    size_t len = strlen(s);
    size_t last = len - 1;
    if(s[last] == '\n')
    {
        s[last] = '\0';
        reverse(s);
    }
    else{
        do
        {
            swap(s, i, last - i);
        } while(++i < ceil(len / 2.0));
    }
}

inline void swap(char *s, int a, int b)
{
    char temp = s[a];
    s[a] = s[b];
    s[b] = temp;
}


double ceil(double l)
{
    if((int) l == l)
        return l;
    return (((int) l) + 1.0);
}
