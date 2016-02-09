// https://projecteuler.net/problem=34
#include <stdlib.h>
#include <stdio.h>
#include <math.h>
int numLen(int i);
int* digits(int i);

int main(int argc, char** argv)
{
    int i = 3;
    int* digits;
    for(;i < 1000; ++i)
    {
        
    }
    return 0;
}

int numLen(int i)
{
    double l = log10(i);
    return l  != (int)l ? (int)ceil(l) : l + 1;
}
int* digits(int i)
{
    int* out = malloc(sizeof(int) * numLen(i));
    int len = 0;
    while(i)
    {
        out[len++] = i % 10;
        i /= 10;
    }
    return out;
}
