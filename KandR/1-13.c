#define MAXWORD 20
#define MAXHEIGHT 20
#define BARCHAR '#'
#include <stdio.h>

int nextLen();

int main(int argc, char** argv)
{
    FILE *in = argc == 1 ? stdin : fopen(argv[1], "r");
    int lens[MAXWORD];
    int i;
    for(i = 0; i < MAXWORD; ++i)
        lens[i] = 0;
    int currLen;
    while((currLen = nextLen(in)))
    {
        lens[currLen]++;
    }


    int highest = 0;
    for(i = 0; i < MAXWORD; i++)
    {
        if(lens[i] > highest)
            highest = lens[i];
    }
    float factor = ((float) highest / MAXHEIGHT) + 1;
    for(i = MAXHEIGHT; i >= 0; i--)
    {
        printf("  ");
        int j = 0;
        for(; j < MAXWORD; j++)
        {
            if(lens[j] / factor > i)
                putchar(BARCHAR);
            else
                putchar(' ');
            putchar(' ');
            putchar(' ');
        }
        printf("\n");
    }
    for(i = 0; i < MAXWORD; i++)
        printf("%3d", i);
    printf("\n");
    return 0;
}

int nextLen(FILE *f)
{
    char nextChar = fgetc(f);
    if(nextChar == EOF)
        return 0;
    do
    {
        if(nextChar == EOF)
            return 0;
    }
    while(!((nextChar = fgetc(f)) > 'A' && nextChar < 'z'));
    int out = 0;
    do 
        out++;
    while((nextChar = fgetc(f)) > 'A' && nextChar < 'z');
    return out;
}
