// 1-24: Write a program to check a C program for rudimentary syntax errors like unbalanced paarentheses, brackets, and braces. 
// Don't forget about quotes, both single and double, excape sequences, and comments.

// Here's my plan. I'm going to make a function that checks a unit for syntax errors, and all that will really do is check it for
// closing punctuation, because that would be bad, make sure that the single and double quotes all match up, and ignore comments and 
// escape squences. If it finds something, it will return an error, and that should bubble back up to the top. If not, it comes 
// out of itself (it's recursive) and keeps moving until EOF. 

#include <stdio.h>
#include <stdlib.h>
#include "hashTable.h"
#define NUMBER_OF_OPPOSITES 3
// I'm guessing single quotes, double quotes, 
// parentheses, curly braces, square brackets, and angle brackets. 
#define NUMBER_OF_SYMBOLS 6

void skipBlockComment(FILE *f);
void skipLineComment(FILE *f);
void initializeStatus(struct ht_Table *table);
int checkSegment(FILE *f, char ending);
void initializeOpposites(struct ht_Table *table);

int main(int argc, char** argv)
{
    if(argc != 2)
    {
        printf("Usage: %s FILE\n", argv[0]);
        return 1;
    }
    FILE *f = fopen(argv[1], "r");
    return checkSegment(f, EOF);
}

int checkSegment(FILE *f, char ending)
{
    struct ht_Table *status = ht_create(NUMBER_OF_SYMBOLS);
    struct ht_Table *opposites = ht_create(NUMBER_OF_OPPOSITES);
    // These two functions were written for a strategy that I'm no longer taking. 
    // I need to rewrite the, but this function should be pretty easy, except a little
    // monotonous. 
    initializeStatus(status);
    initializeOpposites(opposites);
    return 0;
}

void initializeStatus(struct ht_Table *table)
{
    ht_put(table, '(', 0);
    ht_put(table, '\'', 0);
    ht_put(table, '"', 0);
    ht_put(table, '{', 0);
    ht_put(table, '[', 0);
    ht_put(table, '<', 0);
}

void initializeOpposites(struct ht_Table *table)
{
    ht_put(table, '}', (int) '{');
    ht_put(table, ']', (int) '[');
    ht_put(table, '>', (int) '<');
}

void skipBlockComment(FILE *f)
{
    char last = 'a';
    char curr;
    while((curr = fgetc(f)) != '/' && last != '*')
    {
        last = curr;
    }
    fgetc(f);
    return;
}

void skipLineComment(FILE *f)
{
    while(fgetc(f) != '\n')
        ;
    return;
}
