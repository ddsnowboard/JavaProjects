#ifndef SYNC_Q_H
#define SYNC_Q_H
#include <pthread.h>
struct Queue {
    size_t head;
    size_t tail;
    size_t size;
    int* arr;
    pthread_mutex_t mut;
    pthread_cond_t cv;
};

struct Queue* queue_create(int size);

void queue_push(struct Queue* q, int i);

int queue_pop(struct Queue* q);

void queue_destroy(struct Queue* q);
#endif
