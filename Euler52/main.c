#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#define HIGHEST_MULTIPLE 6

int isPermutation(int a, int b);

int main(int argc, char **argv)
{
    // printf("the answer is %d\n", isPermutation(714285, 1428570));
    int i = 100;
    for(;;i++)
    {
        int counter = 2;
        int same = 1;
        for(;counter <= HIGHEST_MULTIPLE; counter++)
        {
            same = same && isPermutation(i, counter * i);
        }
        if(same)
        {
            printf("The number is %d\n", i);
            break;
        }
    }
    return 0;
}

int isPermutation(int a, int b)
{
    // shadowA and shadowB should become a number 
    // where each digit corresponds to how many of
    // a certain number are in a and b. So if shadowA is 12345, 
    // there would be 5 ones, 4 twos, 3 threes, etc.  
    double shadowA = 0, shadowB = 0;
    int power = 1;
    do
    {
        // printf("a is %d and b is %d\n", a, b);
        if(a)
            shadowA += pow(10, a % 10);
        if(b)
            shadowB += pow(10, b % 10);
        power *= 10;
        // printf("shadowa is %f and shadowb is %f\n", shadowA, shadowB);
        // This can't short circuit
    } while((a /= 10) | (b /= 10));
    // printf("shadowa is %f and shadowb is %f\n", shadowA, shadowB);
    return shadowA == shadowB;
}
