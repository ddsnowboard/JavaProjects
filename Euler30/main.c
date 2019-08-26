#include <string.h>
#include <stdio.h>
#include <math.h>
#include <stdlib.h>

#define POWER 5
#define MAX 1200000

int main(void)
{
    int count = 0;
    int total = 0;
    int currCount = 0;
    char* currString = malloc(8);
    char *currLetter = malloc(2 * sizeof(char));
    currLetter[1] = '\0';
    int i = 2;
    for( ;i < MAX; i++)
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
    free(currLetter);
    free(currString);
    return 0;
}
