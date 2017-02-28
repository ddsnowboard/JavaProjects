#include <stdlib.h>
#include "stack.h"
#define DEFAULT_SIZE 10

void stack_push(struct stack *s, char c) {
    if(s->current >= s->capacity) {
        char* newArr = malloc(s->capacity * 2 * sizeof(char));
        int i;
        for(i = 0; i < s->capacity; i++)
            newArr[i] = s->arr[i];
        free(s->arr);
        s->arr = newArr;
        s->capacity *= 2;
    }
    s->arr[s->current++] = c;
}

char stack_pop(struct stack *s) {
    return s->arr[--s->current];
}

int stack_empty(struct stack *s) {
    return s->current == 0;
}

struct stack* stack_create() {
    struct stack* retval = malloc(sizeof(struct stack));
    retval->capacity = DEFAULT_SIZE;
    retval->current = 0;
    retval->arr = malloc(DEFAULT_SIZE * sizeof(char));
    return retval;
}

void stack_free(struct stack *s) {
    free(s->arr);
    free(s);
}
