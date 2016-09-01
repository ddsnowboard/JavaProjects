#include <stdio.h>
#include <stdlib.h>
#define LEFT 0
#define RIGHT 1

#define TRUE 1
#define FALSE 0
// So I found this interview problem on the internet. Take a binary search 
// tree, and turn it into a doubly-linked list. You can only reassign
// pointers, you can't make new nodes. This could be interesting.
// Here's the link: http://codercareer.blogspot.com/2011/09/interview-question-no-1-binary-search.html

struct Node {
    int value;
    struct Node *left;
    struct Node *right;
};

struct Node *initializeTree();
struct Node *getNodePtr();
void printLinkedList(struct Node head);

void convertToLinkedList(struct Node *head);
struct Node *getSmallest(struct Node *head);
struct Node *getBiggest(struct Node *head);

int main(int argc, char** argv)
{
    struct Node *head = initializeTree();
    convertToLinkedList(head);
    while(head->left != NULL)
        head = head->left;
    printLinkedList(*head);
    return 0;
}

struct Node *initializeTree()
{
    struct Node *out;
    struct Node *currLeft = getNodePtr();
    struct Node *currRight = getNodePtr();
    struct Node *currHead = getNodePtr();
    currLeft->value = 4;
    currLeft->left = NULL;
    currLeft->right = NULL;
    currRight->value = 8;
    currRight->left = NULL;
    currRight->right = NULL;
    currHead->value = 6;
    currHead->left = currLeft;
    currHead->right = currRight;
    currLeft = currHead;
    currHead = getNodePtr();
    currHead->value = 10;
    currHead->left = currLeft;
    out = currHead;
    currRight = getNodePtr();
    currRight->value = 14;
    currHead->right = currRight;
    currHead = currRight;
    currRight = getNodePtr();
    currLeft = getNodePtr();
    currRight->value = 16;
    currLeft->value = 12;
    currLeft->right = NULL;
    currLeft->left = NULL;
    currRight->right = NULL;
    currRight->left = NULL;
    currHead->left = currLeft;
    currHead->right = currRight;
    return out;
}

struct Node *getNodePtr()
{
    return (struct Node*) malloc(sizeof(struct Node));
}

void printLinkedList(struct Node head)
{
    while(head.right != NULL)
    {
        printf("%d ", head.value);
        head = *head.right;
    }
}

void convertToLinkedList(struct Node *head)
{
    if(head == NULL)
        return;
    struct Node *nextRight = getSmallest(head->right);
    struct Node *nextLeft = getBiggest(head->left);
    convertToLinkedList(head->left);
    convertToLinkedList(head->right);
    head->left = nextLeft;
    head->right = nextRight;
    if(head->right != NULL)
        head->right->left = head;
    head->left = getBiggest(head->left);
    if(head->left != NULL)
        head->left->right = head;
}

struct Node *getSmallest(struct Node *head)
{
    if(head == NULL)
        return head;
    else if(head->left == NULL)
        return head;
    else {
        struct Node *ret = head;
        while(ret->left != NULL)
            ret = ret->left;
        return ret;
    }
}
struct Node *getBiggest(struct Node *head)
{
    if(head == NULL)
        return head;
    else if(head->right == NULL)
        return head;
    else {
        struct Node *ret = head;
        while(ret->right != NULL)
            ret = ret->right;
        return ret;
    }
}
