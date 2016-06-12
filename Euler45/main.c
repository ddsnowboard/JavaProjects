#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#define LOWEST 40755
#define BASE_ADD 1
#define ARRAY_LENGTH 100000
int fillArrayWithPentagonal(long *arr, int startIdx);
long pentagonal(long i);
int isHexagonal(long i);
int isPentagonal(long i);
long triangle(long i);
int search(long *haystack, int startIdx, int endIdx, long needle);

int main(int argc, char **argv)
{
    long n = 145;
    long out;
    long newest = 0;
    do{
        out = triangle(n++);
        if(isHexagonal(out) && isPentagonal(out))
            newest = out;
    }while(newest <= LOWEST);
    printf("The next number is %ld\n", newest);
    return 0;
}

int isHexagonal(long i)
{
    float inv = (1 + sqrt(1 + 8 * i)) / 4;
    float inv2 = (1 - sqrt(1 + 8 * i)) / 4;
    if(!(inv > 0))
        inv = inv2;
    return inv > 0 && floor(inv) == inv;
}

int isPentagonal(long i)
{
    static int length = 0;
    static long *nums = NULL;
    if(nums == NULL)
    {
        nums = calloc(ARRAY_LENGTH, sizeof(long));
        length = fillArrayWithPentagonal(nums, length);
    }
    while(nums[length - 1] < i)
        length = fillArrayWithPentagonal(nums, length);
    return search(nums, 0, length, i);
}

long pentagonal(long i)
{
    return i * (3 * i - 1) / 2;
}

int fillArrayWithPentagonal(long *arr, int startIdx)
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

long triangle(long i)
{
    return i * (i + 1) / 2;
}

int search(long *haystack, int startIdx, int endIdx, long needle)
{
    int middle = (startIdx + endIdx) / 2;
    long i = haystack[middle];
    if(i == needle)
        return 1;
    else if(haystack[endIdx - 1] < needle)
        return 0;
    else if(startIdx == endIdx - 1)
        return 0;
    else if(i < needle)
        return search(haystack, middle, endIdx, needle);
    else if(i > needle)
        return search(haystack, startIdx, middle, needle);
    printf("Something went wrong with search!");
    exit(2);
    return -1;
}
