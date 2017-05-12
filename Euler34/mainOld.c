// https://projecteuler.net/problem=34
#include <stdlib.h>
#include <stdio.h>
#include <math.h>
int numLen(int i);
int* digits(int i);
int fact(int i);

int main(int argc, char** argv)
{
    int i = 3;
    int* nums;
    int out = 0;
    for(;i < 100000; ++i)
    {
        int len = numLen(i);
        nums = digits(i);
        int c = 0;
        int tot = 0;
        for(; c < len; ++c)
        {
            tot += fact(nums[c]);
        }
        if(tot == i)
            out += i;
        free(nums);
    }
    printf("The total is %d", out);
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
int fact(int i)
{
    //It's ok that this will never work for big numbers because all its inputs will be 
    // one digit. 
    if(i == 1 || i == 0)
        return 1;
    else
        return i * fact(i - 1);
}
