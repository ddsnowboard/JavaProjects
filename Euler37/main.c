#include <stdio.h>
#include <stdlib.h>
#define MAX 10000
/* Use a binary tree to memoize the primes, because why not? */
struct ListNode {
    int value;
    struct ListNode *next;
};
struct ListNode primes;
int isPrime(int i);
int push(struct ListNode *list, int toAdd);
int isTruncatablePrime(int i);

int main(int argc, char** argv)
{
    // Add sample primes
    struct ListNode *ptr = malloc(sizeof(struct ListNode));
    primes = *ptr;
    primes.value = 1;
    primes.next = NULL;
    push(&primes, 2);
    push(&primes, 3);
    push(&primes, 5);
    push(&primes, 7);
    int tot = 0;
    int i = 10;
    for(; i < MAX; ++i)
    {
        if(isTruncatablePrime(i))
        {
            printf("%d\n", i);
            tot += i;
        }
    }
    printf("The sum is %d\n", i);
    return 0;
}

int isPrime(int i)
{
    /* Returns 1 if prime, 0 otherwise */ 

    // This function only works if it gets each number in order
    if(i % 2 == 0)
    {
        return 0;
    }
    else
    {
        // Skip 1 and 2
        struct ListNode walker = *(primes.next -> next);
        while(walker.next && i != walker.value && i % walker.value != 0)
            walker = *walker.next;
        if(walker.next)
        {
            if(i == walker.value)
                return 1;
            else if(i % walker.value == 0)
                return 0;
        }
        else
        {
            push(&primes, i);
            return 1;
        }
    }
    return 0;
}
int push(struct ListNode *list, int toAdd)
{
    struct ListNode *walker = list;
    while(walker -> next)
        walker = walker -> next;
    struct ListNode *next = malloc(sizeof(struct ListNode));
    next -> value = toAdd;
    walker -> next = next;
    return 0;
}
int isTruncatablePrime(int i)
{
    int orig = i;
    int out = isPrime(i);
    long counter = 10e9;
    while(i % counter == i)
        counter /= 10;
    while((out && isPrime(i % counter)) && i)
        counter /= 10;
    i = orig;
    while((out & isPrime(i /= 10)) && i);
    return out;
}
