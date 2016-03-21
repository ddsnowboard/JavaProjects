#include <string.h>
#include <stdio.h>
#include <math.h>
#include <stdlib.h>

#define POWER 5

int main(void)
{
    int count = 0;
    int total = 0;
    int currCount = 0;
    char* currString = malloc(1);
    char *currLetter = malloc(2 * sizeof(char));
    currLetter[1] = '\0';
    int i = 2;
    for( ;i < 1200000; i++)
    {
        sprintf(currString, "%d", i);
        currCount = 0;
        int j = 0;
        for(; j < strlen(currString); j++)
        {
            currLetter[0] = currString[j];
            currCount += pow(atoi(currLetter), POWER);
        }
        if(currCount == i)
        {
            count++;
            total += i;
        }
    }
    printf("There were %d and they add up to %d\n", count, total);
    return 0;
}
