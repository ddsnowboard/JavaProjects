#include <stdlib.h>
#include <math.h>
#include <string.h>
#include <stdio.h>
// https://projecteuler.net/problem=40
#define LENGTH 1000000

int main(int argc, char** argv)
{
    char* number = malloc(LENGTH * sizeof(char));
    int currLength = LENGTH;
    int charAmt = 0;
    int currNumLen = 0;
    char* currNumber = malloc(10);
    int i = 1;
    strcat(number, ".");
    for(;i<LENGTH;i++)
    {
         if(i % 10000 == 0)
             printf("currNumber is %s and i is %d\n", currNumber, i);
        currNumber[0] = '\0';
        sprintf(currNumber, "%d", i);
        currNumLen = strlen(currNumber);
        charAmt += currNumLen;
        if(currNumLen + charAmt >= currLength)
        {
            currLength *= 2;
            number = realloc(number, currLength);
        }
        strcat(number, currNumber);
    } 
    int out = 1;
    int numbers[] = {1, 10, 100, 1000, 10000, 100000, 1000000};
    i = 0; 
    i = 0;
    for(;i<6;i++)
        out *= atoi((char[]) {number[numbers[i]], '\0'});
    printf("The result is %d\n", out);
    return 0;
}
