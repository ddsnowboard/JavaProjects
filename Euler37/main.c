#include <stdio.h>
#include <stdlib.h>
#define MAX 10000000
struct ListNode {
    int value;
    struct ListNode *next;
};
struct ListNode primes;
struct ListNode tip;
int isPrime(int i);
int push(struct ListNode *list, int toAdd);
int isTruncatablePrime(int i);

int main(int argc, char** argv)
{
    // Add sample primes
    struct ListNode *ptr = malloc(sizeof(struct ListNode));
    ptr -> value = 2;
    ptr -> next = NULL;
    primes = *ptr;
    tip = *ptr;
    push(&primes, 3);
    push(&primes, 5);
    push(&primes, 7);

    int tot = 0;
    int i = 11;
    int truncatablePrimes[11];
    int primesLen = 0;
    for(; i < MAX; i += 2)
    {
        if(isTruncatablePrime(i))
        {
            printf("%d\n", i);
            tot += i;
            truncatablePrimes[primesLen++] = i;
        }
    }
    int count = 0;
    printf("Primes are: ");
    for(; count < primesLen; ++count)
    {
        printf("%d, ", truncatablePrimes[count]);
    }
    printf("The sum is %d\n", tot);
    return 0;
}

int isPrime(int i)
{
    /* 
     * Returns 1 if prime, 0 otherwise 
     * This function only works if it gets each number in order
     */ 
    if(i % 2 == 0 || i == 0 || i == 1)
    {
        return 0;
    }
    else
    {
        // Skip 1 and 2
        struct ListNode walker = *(primes.next);
        while(walker.next && i % walker.value != 0)
        {
            walker = *walker.next;
        }
        if(walker.next)
        {
            if(i == walker.value)
                return 1;
            else if(i % walker.value == 0)
                return 0;
        }
        // If there are no numbers higher than this and none of the lower ones are divisible, 
        // it's prime.
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
    struct ListNode *walker = &tip;
    struct ListNode *next = malloc(sizeof(struct ListNode));
    next -> value = toAdd;
    next -> next = NULL;
    walker -> next = next;
    tip = *next;
    return 0;
}
int isTruncatablePrime(int i)
{
    if(i % 10 == 9)
        return 0;
    int orig = i;
    int out = isPrime(i);
    long counter = 10e9;
    if(!out)
        return 0;
    // Bring down counter...
    while(i % counter == i)
        counter /= 10;

    while(counter > 1 && out)
    {
        out = out && isPrime(i % counter);
        counter /= 10;
    }
    i = orig;
    while(i > 10 && (out = out && isPrime(i /= 10)));
    return out;
}
