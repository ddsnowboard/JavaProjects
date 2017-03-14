#include <assert.h>
#include <stdio.h>
#include "stack.h"
#define AMOUNT_TO_PUSH 100000000

int main(int argc, char** argv) {
    struct stack* s = stack_create();
    stack_push(s, 'a');
    stack_push(s, 'p');
    stack_push(s, 'p');
    stack_push(s, 'l');
    stack_push(s, 'e');
    printf("Testing ordering...\n");
    assert(stack_pop(s) == 'e');
    assert(stack_pop(s) == 'l');
    assert(stack_pop(s) == 'p');
    assert(stack_pop(s) == 'p');
    assert(stack_pop(s) == 'a');

    printf("Worked\n");
    stack_free(s);


    printf("testing realloc...\n");
    s = stack_create();
    int i;
    for(i = 0; i < AMOUNT_TO_PUSH; i++)
    {
        stack_push(s, 'a');
    }
    int counter = 0;
    while(!stack_empty(s)) {
        stack_pop(s);
        counter++;
    }
    assert(counter == AMOUNT_TO_PUSH);
    printf("Worked\n");
    stack_free(s);
    return 0;
}
