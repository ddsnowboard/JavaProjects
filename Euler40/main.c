#include <stdlib.h>
#include <string.h>
#include <stdio.h>
// https://projecteuler.net/problem=40
// #define LENGTH 1000000
#define LENGTH 30

int main(int argc, char** argv)
{
    char* number = malloc(LENGTH + 10);
    char* currNumber;
    int i = 0;
    for(;i<LENGTH;i++)
    {
        currNumber = malloc(5);
        sprintf(currNumber, "%d", i);
        strcat(number, currNumber);
    } 
    printf("%s\n", number);
    return 0;
}
