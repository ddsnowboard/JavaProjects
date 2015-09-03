#include <stdlib.h>
#include <math.h>
#include <string.h>
#include <stdio.h>
// https://projecteuler.net/problem=40
#define LENGTH 1000000
// #define LENGTH 100

char* mystrcat(char*, char*);
int main(int argc, char** argv)
{
    char* number = malloc(LENGTH);
    int currLength = LENGTH;
    int charAmt = 1;
    int currNumLen = 0;
    char* currNumber = malloc(10);
    char* currentDestination = number;
    int i = 1;
    number[0] = '.';
    number[1] = '\0';
    for(;i<LENGTH;i++)
    {
        //  if(i % 10000 == 0)
        //      printf("currNumber is %s and i is %d\n", currNumber, i);
        currNumber[0] = '\0';
        sprintf(currNumber, "%d", i);
        currNumLen = strlen(currNumber);
        charAmt += currNumLen;
        if(charAmt >= currLength)
        {
            currLength *= 2;
            number = realloc(number, currLength);
            currentDestination = number;
            // printf("Realloced; now %s\n", number);
        }
        // printf("currLength is %d and charAmt is %d\n", currLength, charAmt);
        currentDestination = mystrcat(currentDestination, currNumber);
        // printf("%s\n", number);
    } 
    int out = 1;
    int numbers[] = {1, 10, 100, 1000, 10000, 100000, 1000000};
    i = 0; 
    for(;i < 6; i++)
    {
        // printf("Multiplying %c\n", number[numbers[i]]);
        out *= atoi((char[]) {number[numbers[i]], '\0'});
    }
    printf("The result is %d\n", out);
    return 0;
}
char* mystrcat(char* dest, char* src)
{
    if(dest != '\0'){
        while(*++dest);
    }
    while(*dest++ = *src++);
    // I think that --dest is just pointing to \0 so it won't work. I'm not sure though. 
    return dest - 2;
}
