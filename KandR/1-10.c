
#include <stdio.h>

// Unescape things

int main(int argc, char** argv)
{
    char c;
    while((c = getchar()) != EOF)
    {
        if(c == '\t')
        {
            putchar('\\');
            putchar('t');
        }
        else if(c == '\\')
        {
            putchar('\\');
            putchar('\\');
        }
        else if(c == '\b')
        {
            putchar('\\');
            putchar('b');
        }
        else
            putchar(c);
    }
    return 0;
}
