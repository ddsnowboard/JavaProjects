// Evidently this way won't actually work. It is too dependent on 
// order, which isn't good. I don't know what to do though. I have this idea that 
// I could do something where I start by building all the way up to the target
// using the smallest denomination, and then I go through the array of 1's and 
// see if all the subarrays can be replaced by a single other coin. I don't really 
// know what I'd do past that though. 
//
// Another option would be to keep doing what I was doing but instead of 
// having a set of linked lists, I would create a python counter, basically, 
// and put those in a set. I don't know what I'll do, but I bet that could work. 
// Slowly

#include <cstdio>
#include <unordered_set>
#include <iostream>
#include "Node.h"

#define NUMBER_OF_DENOMINATIONS 8
#define STARTING_AMOUNT 100
using namespace std;

int countWays(int remaining, int currDenomination, const int denominations[NUMBER_OF_DENOMINATIONS], std::unordered_set<Node*>* heads, Node* lastNode);

int main(int argc, char** argv)
{
    int DENOMINATIONS[] = {1, 2, 5, 10, 20, 50, 100, 200};
    std::unordered_set<Node*>* set_o = new std::unordered_set<Node*>();
    for(int i = 0; i < NUMBER_OF_DENOMINATIONS; i++)
        countWays(STARTING_AMOUNT, DENOMINATIONS[i], DENOMINATIONS, set_o, NULL);
    printf("%ld\n", set_o->size());
    return 0;
}

int countWays(int remaining, int currDenomination, const int denominations[NUMBER_OF_DENOMINATIONS], std::unordered_set<Node*>* heads, Node* lastNode)
{
    if(remaining < 0)
    {
        return 0;
    }
    else if(remaining == 0)
    {
        Node n;
        n.value = currDenomination;
        n.next = lastNode;
        heads->insert(&n);
        return 1;
    }
    else
    {
        // cout << "Down to " << remaining << "\n";
        Node n;
        n.value = currDenomination;
        n.next = lastNode;
        int out = 0;
        for(int i = 0; i < NUMBER_OF_DENOMINATIONS; i++)
        {
            out += countWays(remaining - currDenomination, denominations[i], denominations, heads, &n);
        }
        return out;
    }

}
