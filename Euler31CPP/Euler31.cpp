#include <cstdio>
#include <unordered_set>
#include <iostream>
#include "Node.h"
using namespace std;

int countWays(int remaining, int currDenomination, const int denominations[8], std::unordered_set<Node*> heads, Node lastNode);

int main(int argc, char** argv)
{
    int DENOMINATIONS[] = {1,2,5,10,20,50,100,200};
    Node n1;
    n1.value = 5;
    Node n2;
    n2.value = 44;
    Node n3;
    n3.value = 32;
    n1.next = &n2;
    n2.next = &n3;
    n3.next = NULL;
    Node* curr = &n1;
    // while(curr != NULL)
    // {
        // cout << curr -> value << '\n';
        // curr = curr -> next;
    // }
    std::unordered_set<Node*>* set_o = new std::unordered_set<Node*>();
    printf("%d\n", countWays(0,0, DENOMINATIONS, *set_o, n3));
    return 0;
}

int countWays(int remaining, int currDenomination, const int denominations[8], std::unordered_set<Node*> heads, Node lastNode)
{
    return 111;
}
