#include <stdlib.h>
#include <math.h>
#include <string.h>
#include <stdio.h>
// https://projecteuler.net/problem=40
// #define LENGTH 1000000
#define LENGTH 300000

int main(int argc, char** argv)
{
    char* number = malloc(LENGTH);
    char* holder = malloc(LENGTH);
    int currLength = LENGTH;
    char* currNumber;
    int i = 1;
    for(;i<LENGTH;i++)
    {
        printf("We're on %d\n", i);
        printf("mallocing %d\n", (int) (log(i) / log(10)) + 2);
        currNumber = malloc(((int) log(i) / log(10)) + 1);
        printf("Got past currnumber\n");
        sprintf(currNumber, "%d", i);
        if(strlen(currNumber) + strlen(number) > currLength)
        {
            strcpy(holder, currNumber);
            free(currNumber);
            currNumber = malloc(currLength * 2);
            currLength *= 2;
            strcpy(currNumber, holder);
        }
        strcat(number, currNumber);
        free(currNumber);
    } 
    printf("%s\n", number);
    return 0;
}
