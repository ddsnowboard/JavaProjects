#include <stdio.h>
#include <stdlib.h>
#define MAX 10000
/* Use a binary tree to memoize the primes, because why not? */
struct Node {
    int value;
    struct Node *right;
    struct Node *left;
};
extern struct Node head;
int isPrime(int i);
int treeSearch(int i);
int treeAdd(int i);

int main(int argc, char** argv)
{
    int tot = 0;
    int i = 0;
    for(; i < MAX; ++i)
    {
        if(isTruncatablePrime(i))
        {
            printf("%d", i);
            tot += i;
        }
        printf("The sum is %d", i);
    }
    return 0;
}

int isPrime(int i)
{
    if(i % 2 == 0)
    {
        return 0;
    }
    else if(treeSearch(i))
    {
        return 1;
    }
    else 
    {
        struct Node walker = head;
    }
    return 0;
}
