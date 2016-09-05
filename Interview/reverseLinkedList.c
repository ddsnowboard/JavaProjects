#include <stdio.h>
#include <stdlib.h>

struct Node {
    int value;
    struct Node *next;
};
void printLinkedList(struct Node *head);
struct Node *reverseLinkedList(struct Node *head);
struct Node *initializeList();

int main(int argc, char **argv)
{
    struct Node *head = initializeList();
    head = reverseLinkedList(head);
    printLinkedList(head);
    return 0;
}


struct Node *initializeList()
{
    const int NUMBERS[] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    const int LENGTH = 10;
    int i;
    struct Node *head = (struct Node *) malloc(sizeof(struct Node));
    struct Node *currNode = head;
    struct Node *nextNode = (struct Node *) malloc(sizeof(struct Node));
    struct Node *laggard;
    for(i = 0; i < LENGTH; ++i)
    {
        currNode->value = NUMBERS[i];
        currNode->next = nextNode;
        laggard = currNode;
        currNode = nextNode;
        nextNode = (struct Node *) malloc(sizeof(struct Node));
    }
    free((void *) nextNode);
    laggard->next = NULL;
    return head;
}

void printLinkedList(struct Node *head)
{
    struct Node *curr = head;
    do
    {
        printf("%d\n", curr->value);
    } while((curr = curr->next) != NULL);
}

struct Node *reverseLinkedList(struct Node *head)
{
    if(head->next == NULL)
        return head;

    struct Node *curr = NULL;
    struct Node *next = head;
    do{
        struct Node *upcoming = next->next;
        next->next = curr;
        curr = next;
        next = upcoming;
    } while(next);
    return curr;
}
