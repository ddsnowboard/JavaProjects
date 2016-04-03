#include <stdio.h>

int main(int argc, char** argv)
{
    char c;
    int inBlank = 0;
    while((c = getchar()) != EOF)
    {
        if(c == ' ' || c == '\t')
        {
            if(!inBlank)
            {
                inBlank = 1;
                putchar(' ');
            }
        }
        else
        {
            inBlank = 0;
            putchar(c);
        }
    }
    return 0;
}
