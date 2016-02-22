// https://projecteuler.net/problem=34
#include <stdlib.h>
#include <stdio.h>
#include <math.h>
#include <pthread.h>
#define MAX 9999999 
int numLen(int i);
int* digits(int i);
int fact(int i);
void* test(void* s);
struct Inputs {
    int i;
    int* tot;
    int* counter;
};

int main(int argc, char** argv)
{
    int i = 3;
    int out = 0;
    int counter = 0;
    pthread_t* threads = malloc(MAX * sizeof(pthread_t));
    for(; i < MAX; ++i)
    {
        pthread_t thread;
        struct Inputs input;
        input.i = i;
        input.tot = &out;
        input.counter = &counter;
        pthread_create(&thread, NULL, &test, (void *)&input);
    }
    i = 0;
    for(; i < MAX; ++i)
    {
        pthread_join(threads[i], NULL);
    }
    printf("The sum is %d\n", out);
    return 0;
}

void* test(void* s)
{
    struct Inputs* inputs = (struct Inputs *)s;
    int i = inputs -> i;
    int* nums = digits(i);
    int len = numLen(i);
    int c = 0;
    int tot = 0;
    for(; c < len;++c)
    {
        tot += fact(nums[c]);
    }
    if(tot == i)
    {
        *(inputs -> tot) += i;
    }
    ++(*(inputs -> counter));
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
    // It's ok that this will never work for big numbers because all its inputs will be 
    // one digit. 
    if(i == 1 || i == 0)
        return 1;
    else
        return i * fact(i - 1);
}
