#include <stdlib.h>
#include <stdio.h>
#include <pthread.h>
#include "syncQueue.h"
#define MAX 1000000
#define NUM_THREADS 1


void* threadFunc(void* in);

int main(int argc, char** argv)
{
    pthread_t threads[NUM_THREADS];
    struct Queue* q = queue_create(4 * NUM_THREADS);
    for(int i = 0; i < NUM_THREADS; i++) {
        pthread_create(&threads[i], NULL, &threadFunc, (void *) q);
    }

    for(int i = 0; i < MAX; i++) {
        queue_push(q, i);
    }

    for(int i = 0; i < NUM_THREADS; i++) {
        queue_push(q, -1);
    }

    for(int i = 0; i < NUM_THREADS; i++) {
        pthread_join(threads[i], NULL);
    }
    return 0;
}

// Returns 0 if Collatz Conjecture works, a number if otherwise.
int collatz(int i)
{
    if(i <= 0)
    {
        return 0;
    }
    // These are to make sure that we aren't just going in a circle.
    int newest = 0;
    int secondNewest = 0;
    // SOMEHOW WE KEEP GETTING A NEGATIVE NUMBER HERE. I'M CONFUSED
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
            return 42;
        }
        if(i == newest || i == secondNewest)
        {
            printf("Doesn't work for %d", i);
            return i;
        }
        else 
        {
            secondNewest = newest, newest = i;
        }
    }
    return 0;
}

void* threadFunc(void* in) {
    struct Queue* q = in;
    int i;
    do {
        i = queue_pop(q);
        printf("Popped off an %d\n", i);
        collatz(i);
    } while(i != -1);

    return NULL;
}
