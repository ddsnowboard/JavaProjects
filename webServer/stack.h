#ifndef STACK_H
#define STACK_H
struct stack {
    int capacity;
    int current;
    char* arr;
};
void stack_push(struct stack *s, char c);
char stack_pop(struct stack *s);
int stack_isEmpty(struct stack *s);
struct stack* stack_create();
void stack_free(struct stack* s);
#endif
