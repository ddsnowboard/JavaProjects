#include <stdio.h>
#include <stdlib.h>

struct Node {
    int value;
    struct Node* last;
    struct Node* next;
};

struct Node* reverseDLinkedList(struct Node* head);
void printDLinkedList(struct Node* head);
struct Node* newLink(int value, struct Node* left, struct Node* right);
void dLinkedListAppend(struct Node* head, int value);
struct Node* arrToDLinkedList(int* arr, int len);

int main(int argc, char** argv)
{
    int VALUES[] = {1, 2, 3, 4, 5, 6, 4, 2, 35, 16, 12, 4, 55};
    struct Node* head = arrToDLinkedList(VALUES, sizeof(VALUES) / sizeof(VALUES[0]));
    printDLinkedList(head);
    struct Node* newHead = reverseDLinkedList(head);
    printDLinkedList(newHead);
    return 0;
}

struct Node* newLink(int value, struct Node* last, struct Node* next)
{
    struct Node* out = (struct Node*) malloc(sizeof(struct Node));
    out->last = last;
    out->next = next;
    out->value = value;
    return out;
}

void dLinkedListAppend(struct Node* head, int value)
{
    if(head->next == NULL)
        head->next = newLink(value, head, NULL);
    else
        dLinkedListAppend(head->next, value);
}
struct Node* arrToDLinkedList(int* arr, int len)
{
    struct Node* head = newLink(arr[0], NULL, NULL);
    // Can't be Schlemiel
    struct Node* walker = head;
    for(int i = 1; i < len; i++)
    {
        dLinkedListAppend(walker, arr[i]);
        walker = head->next;
    }
    return head;
}

void printDLinkedList(struct Node* head)
{
    printf("%d\n", head->value);
    if(head->next != NULL)
        printDLinkedList(head->next);
    else
        printf("\n");
}

struct Node* reverseDLinkedList(struct Node* head)
{
    struct Node* temp = head->next;
    head->next = head->last;
    head->last = temp;
    if(head->last == NULL)
        return head;
    return reverseDLinkedList(head->last);
}
