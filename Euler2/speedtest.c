/*
 * This is a C version so I can performance test the Racket version
 */
#include <stdio.h>

int main(int argc, char** argv)
{
    long long max;
    if(argc == 2)
        max = atoll(argv[1]);
    else
        max = 4000000l;
    printf("%d\n", max);
    unsigned long long int a = 1;
    unsigned long long int b = 2;
    unsigned long long int temp;
    // Would be 1 + 2 = 3, but 1 is odd 
    unsigned long long int total = 2;
    while(a + b < max)
    {
        temp = b;
        b += a;
        a = temp;
        if(b % 2 == 0)
            total += b;
    }
    printf("%d\n", total);
}
