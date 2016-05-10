#include <stdlib.h>
#include <stdio.h>
#include <pthread.h>
#define MAX 1000000
#define NUM_THREADS 8


void *collatz(void *info);

struct Info {
    int i;
};
int main(int argc, char** argv)
{
    pthread_t threads[NUM_THREADS];
    struct Info infos[NUM_THREADS];
    int i = 0;
    while(i <= MAX)
    {
        int processor;
        for(processor = 0; processor < NUM_THREADS; processor++)
        {
            infos[processor].i = i;
            pthread_create(&threads[processor], NULL, &collatz, (void *) &infos[processor]);
            i++;
        }
        for(processor = 0; processor < NUM_THREADS; processor++)
        {
            pthread_join(threads[processor], NULL);
        }
    }
    return 0;
}

// Returns 0 if Collatz Conjecture works, a number if otherwise.
void *collatz(void *info)
{
    struct Info *input = (struct Info *)info;
    int i = input->i;
    if(i == 113383)
        printf("We hit it");
    if(i == 0)
    {
        return NULL;
    }
    printf("number is %d\n", i);
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
            return NULL;
        }
        if(i == newest || i == secondNewest)
        {
            printf("Doesn't work for %d", input->i);
        }
        else 
        {
            secondNewest = newest, newest = i;
        }
    }
    return NULL;
}
