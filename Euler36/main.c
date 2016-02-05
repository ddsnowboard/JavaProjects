// https://projecteuler.net/problem=36
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
char* reverse(char* s);
char* itoa(int i);
char* itobin(int i);

int main(int argc, char** argv) 
{
    int tot = 0;
    char* currNum;
    for(int i = 0; i <= 1000000; ++i)
    {
        currNum = malloc(10);
        sprintf(currNum, "%d", i);
        if(strcmp(currNum, reverse(currNum)) == 0)
        {
            tot += i;
        }
    }
}
char* reverse(char* s)
{
    int len = strlen(s);
    char* out = malloc(len + 1);
    int j = 0;
    for(int i = len - 1; i >= 0; --i, ++j)
    {
        out[j] = s[i];
    }
    out[j + 1] = '\0';
    return out;
}
