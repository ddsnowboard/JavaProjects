#include <stdlib.h>
#include <math.h>
#include <string.h>
#include <stdio.h>
// https://projecteuler.net/problem=40
#define LENGTH 1000000

int main(int argc, char** argv)
{
    char number[LENGTH];
    char holder[LENGTH];
    int currLength = LENGTH;
    char* currNumber;
    int i = 1;
    int currentLengthOfNumber = 1;
    for(;i<LENGTH;i++)
    {
        // It's plus two because it will round down (that's one) and I want to have some cushion.
        if((int) (log(i) / log(10)) + 2 > currentLengthOfNumber)
        {
            currNumber = realloc(currNumber, ++currentLengthOfNumber);
            printf("Realloced\n");
        }
        if(i % 10000 == 0)
            printf("currNumber is %s and i is %d\n", currNumber, i);
        sprintf(currNumber, "%d", i);
        if(strlen(currNumber) + strlen(number) > currLength)
        {
            strcpy(holder, currNumber);
            free(currNumber);
            currLength *= 2;
            currNumber = malloc(currLength);
            strcpy(currNumber, holder);
        }
        strcat(number, currNumber);
    } 
    printf("%s\n", number);
    return 0;
}
