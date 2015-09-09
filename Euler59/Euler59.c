#include <stdio.h>
#include <string.h>
#include <stdlib.h>
int letterCount(FILE*);

int main(int argc, char** argv)
{
    FILE *in = fopen(argv[1], "r");
    int length = letterCount(in);
    int* letters = malloc(sizeof(int) * length);
    int currentIndexInLetters = 0;
    int currentIndexInPair = 0;
    char *pair = malloc(3);
    pair[2] = '\0';
    char nextChar;
    while((nextChar = getc(in)))
    {
        if(nextChar != '\n')
        {
            if(nextChar != ',')
            {
                pair[currentIndexInPair++] = nextChar;
            }
            else
            {
                currentIndexInPair = 0;
                letters[currentIndexInLetters++] = atoi(pair);
            }
        }
    }
    return 0;
}

int letterCount(FILE *f)
{
    char curr;
    int count = 0;
    while((curr = getc(f)))
    {
        if(curr == ',')
        {
            count++;
        }
    }
    return count + 1;
}
