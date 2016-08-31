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

int main(int argc, char** argv)
{
    return 0;
}
