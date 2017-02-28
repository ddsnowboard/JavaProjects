#include <stdio.h>
#include "stack.h"

int main(int argc, char** argv) {
    struct stack* s = stack_create();
    stack_push(s, 'a');
    stack_push(s, 'p');
    stack_push(s, 'p');
    stack_push(s, 'l');
    stack_push(s, 'e');
    while(!stack_empty(s)) 
        printf("%c", stack_pop(s));
    printf("\n");
    stack_free(s);
    s = stack_create();
    int i;
    for(i = 0; i < 1000; i++)
    {
        stack_push(s, 'a');
    }
    int counter = 0;
    while(!stack_empty(s)) {
        printf("Popped off a %c, at count %d\n", stack_pop(s), counter++);
    }
    stack_free(s);
    return 0;
}
