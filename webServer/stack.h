#ifndef STACK_H
#define STACK_H
struct stack {
    int capacity;
    int current;
    char* arr;
};
void push(struct stack *s, char c);
char pop(struct stack *s);
int isEmpty(struct stack *s);
struct stack* stack_create();
void stack_free(struct stack* s);
#endif
