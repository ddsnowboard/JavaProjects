#include <unordered_set>
#include <iostream>
#include "Node.h"
using namespace std;

int countWays(int remaining, int currDenomination, int denominations[8], std::unordered_set<struct Link> heads, struct Link lastNode);
int main(int argc, char** argv)
{
    const int DENOMINATIONS[] = {1,2,5,10,20,50,100,200};
    Node n1;
    n1.value = 5;
    Node n2;
    n2.value = 44;
    Node n3;
    n3.value = 32;
    n1.next = &n2;
    n2.next = &n3;
    n3.next = NULL;
    // for(int i = 0; i < sizeof(DENOMINATIONS) / sizeof(int); i++)
    // {
        // cout << DENOMINATIONS[i] << '\n';
    // }
    Node* curr = &n1;
    while(curr != NULL)
    {
        cout << curr -> value << '\n';
        curr = curr -> next;
    }
    return 0;
}

int countWays(int remaining, int currDenomination, int denominations[8], std::unordered_set<Link> heads, Link lastNode)
{
    return 111;
}
