#include <stdlib.h>
#include <stdio.h>
#define INPUT_FILE "input.txt"
#define MAX_NAME_LENGTH 10

struct Node {
    char *val;
    struct Node *next;
};
struct Node *head;

void push(char *val);
char* get(int idx);
void printList();

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
    return 0; 
}

void push(char *val)
{
    struct Node *curr = head;
    struct Node *new = (struct Node *) malloc(sizeof(struct Node));
    new->val = val;
    new->next = NULL;
    if(head == NULL)
    {
        printf("Head was null; reassigning to %s\n", val);
        head = new;
        return;
    }
    while(curr->next != NULL)
        curr = curr->next;
    curr->next = new;
}

char* get(int idx)
{
    struct Node *curr = head;
    while(idx > 0)
    {
        idx--;
        curr = curr->next;
    }
    return curr->val;
}

void printList(void)
{
    struct Node *curr = head;
    while(curr->next != NULL)
    {
        printf("%s\n", curr->val);
        curr = curr->next;
    }
}
