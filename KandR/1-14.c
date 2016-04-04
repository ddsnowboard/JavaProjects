#define MAXWORD 20
#define MAXHEIGHT 20
#define BARCHAR '#'
#include <stdio.h>

int main(int argc, char** argv)
{
    FILE *in = argc == 1 ? stdin : fopen(argv[1], "r");
    // http://i.imgur.com/hvhMtiK.jpg
    int lens['z' + 1];
    int i;
    for(i = 0; i <= 'z'; ++i)
        lens[i] = 0;
    char curr;
    while((curr = fgetc(in)) != EOF)
        lens[(unsigned int) curr]++;


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
        int j = 'A';
        for(; j <= 'z'; j++)
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
    for(i = 'A'; i <= 'z'; i++)
        printf("  %c", (char) i);
    printf("\n");
    return 0;
}
