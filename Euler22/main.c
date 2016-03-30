#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#define INPUT_FILE "input.txt"
#define MAX_NAME_LENGTH 10

struct Node {
    char *val;
    struct Node *prev;
    struct Node *next;
};
struct Node *head;
struct Node *tail;

void push(char *val);
char* get(int idx);
void printList();
void swap(struct Node *a, struct Node *b);
void swapIdx(int a, int b);
int compare(struct Node *a, struct Node *b);
void sort(struct Node *first, struct Node *last);
int length(struct Node *list);
int sliceLength(struct Node *first, struct Node *last);

int main(int argc, char** argv)
{
    FILE* in = fopen(INPUT_FILE, "r");
    char next;
    char* currString = calloc(MAX_NAME_LENGTH, sizeof(char));
    int currStrLen = 0;
    while((next = fgetc(in)) != EOF)
    {
        if(next == '"')
        {
            continue;
        }
        else if(next == ',')
        {
            push(currString);
            currString = calloc(MAX_NAME_LENGTH, sizeof(char));
            currStrLen = 0;
        }
        else
        {
            currString[currStrLen++] = next;
        }
    }
    sort(head, tail);
    // printList();
    return 0; 
}

void push(char *val)
{
    struct Node *new = (struct Node *) malloc(sizeof(struct Node));
    new->val = val;
    new->next = NULL;
    if(head == NULL)
    {
        new->prev = NULL;
        head = new;
        tail = new;
        return;
    }
    else{
        new->prev = tail;
        tail->next = new;
        tail = new;
    }
}

char* get(int idx)
{
    if(idx < 0)
    {
        idx *= -1;
        struct Node *curr = tail;
        while(idx > 0)
        {
            idx--;
            curr = curr->prev;
        }
        return curr->val;
    }
    else{
        struct Node *curr = head;
        while(idx > 0)
        {
            idx--;
            curr = curr->next;
        }
        return curr->val;
    }
}

void printList(void)
{
    printf("First, forward\n");
    struct Node *curr = head;
    while((curr = curr->next) != NULL)
    {
        printf("%s\n", curr->val);
    }
    printf("\n\n\n\nNow, backward\n");
    curr = tail;
    while(curr->prev != NULL)
    {
        printf("%s\n", curr->val);
        curr = curr->prev;
    }
}

void swap(struct Node *a, struct Node *b)
{
    char* temp = a->val;
    a->val = b->val;
    b->val = temp;
}

void swapIdx(int a, int b)
{
    struct Node *nA = head;
    struct Node *nB = head;

    while(a > 0)
    {
        nA = nA->next;
        a--;
    }
    while(b > 0)
    {
        nB = nB->next;
        b--;
    }
    swap(nA, nB);
}

int compare(struct Node *a, struct Node *b)
{
    return strcmp(a->val, b->val);
}

int length(struct Node *list)
{
    if(list == NULL)
        return 0;
    int out = 0;
    struct Node *curr = list;
    do
        out++;
    while((curr = curr->next) != NULL);
    return out;
}

int sliceLength(struct Node *first, struct Node *last)
{
    if(first == last)
        return 1;
    int out = 0;
    struct Node *curr = first;
    do
        out++;
    while((curr = curr->next) != last);
    return out;
}

void sort(struct Node *first, struct Node *last)
{
    int sorted = 1;
    struct Node *curr = first->next;
    do{
        sorted = 1;
        curr = first->next;
        while(curr->next != NULL)
        {
            if(curr == NULL)
                printf("Something bad happened!\n");
            if(compare(curr->prev, curr) < 0)
            {
                swap(curr->prev, curr);
                sorted = 0;
            }
            curr = curr->next;
        }
    } while(!sorted);
}
