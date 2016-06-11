#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#define LOWEST 40755
#define BASE_ADD 1
#define ARRAY_LENGTH 100000
int search(int *haystack, int length, int needle);
int fillArrayWithPentagonal(int *arr, int startIdx);
int pentagonal(int i);
int isHexagonal(int i);
int isPentagonal(int i);
int triangle(int i);

int main(int argc, char **argv)
{
    int n = 1;
    int out;
    int newest = 0;
    do{
        out = triangle(n++);
        if(out == 157557877)
            printf("out is %d, and n was %d", out, n - 1);
        if(isHexagonal(out) && isPentagonal(out))
            newest = out;
    }while(newest <= LOWEST);
    printf("The next number is %d\n", newest);
    return 0;
}

int isHexagonal(int i)
{
    float inv = (1 + sqrt(1 + 8 * i)) / 4;
    float inv2 = (1 - sqrt(1 + 8 * i)) / 4;
    if(!(inv > 0))
        inv = inv2;
    return inv > 0 && floor(inv) == inv;
}

int isPentagonal(int i)
{
    static int length = 0;
    static int *nums = NULL;
    if(nums == NULL)
    {
        nums = calloc(ARRAY_LENGTH, sizeof(int));
        length = fillArrayWithPentagonal(nums, length);
    }
    while(nums[length - 1] < i)
        length = fillArrayWithPentagonal(nums, length);
    return search(nums, length, i);
}

int pentagonal(int i)
{
    return i * (3 * i - 1) / 2;
}

int fillArrayWithPentagonal(int *arr, int startIdx)
{
    int currIdx = startIdx;
    if(startIdx + BASE_ADD > ARRAY_LENGTH)
    {
        printf("You need more memory allocated!");
        exit(1);
    }
    for(; currIdx < startIdx + BASE_ADD; ++currIdx)
        arr[currIdx] = pentagonal(currIdx);
    return currIdx;
}

int triangle(int i)
{
    return i * (i + 1) / 2;
}

int search(int *haystack, int length, int needle)
{
    int i;
    for(i = *haystack; length-- > 0; i = *++haystack)
    {
        if(i == needle)
            return 1;
    }
    return 0;
}
