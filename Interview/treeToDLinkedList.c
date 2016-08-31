#include <stdio.h>
#include <stdlib.h>

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
int main(int argc, char** argv)
{
    struct Node *head = initializeTree();
    // This could be harder than I thought
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
    while(head->right != NULL)
    {
        printf("%d ", head->value);
        head = head->right;
    }
}
