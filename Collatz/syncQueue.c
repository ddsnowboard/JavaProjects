#include <stdlib.h>
#include <stdio.h>
#include <pthread.h>
#include "syncQueue.h"

struct Queue* queue_create(int size) {
    struct Queue* out = malloc(sizeof(struct Queue));
    out->head = 0;
    out->tail = 0;
    out->size = size;
    out->arr = malloc(sizeof(int) * size);
    pthread_mutex_init(&out->mut, NULL);
    pthread_cond_init(&out->cv, NULL);
    return out;
}

void queue_push(struct Queue* q, int i) {
    pthread_mutex_lock(&q->mut);

    while(q->tail - q->head >= q->size) {
        printf("Full q\n");
        pthread_cond_wait(&q->cv, &q->mut);
        printf("Freed q\n");
    }

    q->arr[q->tail % q->size] = i;
    q->tail = (q->tail + 1) % q->size;
    pthread_cond_broadcast(&q->cv);
    pthread_mutex_unlock(&q->mut);
}

int queue_pop(struct Queue* q) {
    pthread_mutex_lock(&q->mut);
    while(q->tail - q->head <= 0) {
        printf("Empty q\n");
        pthread_cond_wait(&q->cv, &q->mut);
        printf("Filled q\n");
    }
    int out = q->arr[q->head % q->size];
    q->head = (q->head + 1) % q->size;
    pthread_cond_broadcast(&q->cv);
    pthread_mutex_unlock(&q->mut);
    return out;
}

void queue_destroy(struct Queue* q) {
    // God help you if anyone still has the mutex
    free(q->arr);
    free(q);
}

