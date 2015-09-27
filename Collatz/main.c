#include <stdio.h>
#define MAX 1000000000


int collatz(int);

// int main(int argc, char** argv)
int main(int argc, char** argv)
{
    int i;
    for(i = 0;i<MAX;i++)
    {
        if(collatz(i) != 0)
        {
            printf("%d returned 1\n", i);
        }
    }
    return 0;
}

// Returns 0 if Collatz Conjecture works, a number if otherwise.
int collatz(int i)
{
    return 0;
    int originali = i;
    if(i == 0)
        return 1;
    // These are to make sure that we aren't just going in a circle.
    int newest = 0;
    int secondNewest = 0;
    while(i != 1)
    {
        if(i % 2 == 0)
        {
            i /= 2;
        }
        else if(i % 2 != 0)
        {
            i = 3 * i + 1;
        }
        else
        {
            printf("Something really bad just happened...");
            return 124;
        }
        if(i == newest || i == secondNewest)
        {
            printf("Doesn't work for %d", originali);
            return 1;
        }
        else 
        {
            secondNewest = newest, newest = i;
        }
    }
    return 0;
}
