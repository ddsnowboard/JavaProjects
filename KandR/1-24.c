// 1-24: Write a program to check a C program for rudimentary syntax errors like unbalanced parentheses, brackets, and braces. 
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
int search(char *haystack, int length, char needle);
int skipString(FILE *f, char start);
void skipLineComment(FILE *f);
void initializeStatus(struct ht_Table *table);
int checkSegment(FILE *f, char ending);
void initializeOpposites(struct ht_Table *table);

int main(int argc, char** argv)
{
    FILE *f = fopen(argv[1], "r");
    if(argc != 2 || f == NULL)
    {
        printf("Usage: %s FILE\n", argv[0]);
        return 1;
    }
    int output = checkSegment(f, EOF);
    if(output != 0)
        printf("There was an error!\n");
    return output;
}

int checkSegment(FILE *f, char ending)
{
    struct ht_Table *status = ht_create(NUMBER_OF_SYMBOLS);
    struct ht_Table *opposites = ht_create(NUMBER_OF_OPPOSITES);
    initializeStatus(status);
    initializeOpposites(opposites);
    char CLOSERS[] = {')', '>', ']', '}'};
    char OPENERS[] = {'(', '<', '[', '{'};
    const int LISTS_LENGTH = 4;
    char last = 'a';
    char curr;
    while((curr = fgetc(f)) != EOF)
    {
        printf("%c", curr);
        if(curr == '#')
            skipLineComment(f);
        else if(curr == '*' && last == '/')
            skipBlockComment(f);
        else if(curr == '/' && last == '/')
            skipLineComment(f);
        else if(curr == '\'' || curr == '"')
        {
            if(skipString(f, curr) != 0)
                return 1;
        }
        else if(search(OPENERS, LISTS_LENGTH, curr) != -1)
        {
            (*ht_get(status, curr))++;
        }
        // Struct pointer references screw up the normal algorithm. Don't tell Tim Peters or Linus Torvalds
        else if(curr == '>' && last == '-')
        {}
        else if(search(CLOSERS, LISTS_LENGTH, curr) != -1)
        {
            char correspondingOpener = (char) (*ht_get(opposites, curr));
            int *num = ht_get(status, correspondingOpener);
            if(*num == 0)
            {
                printf("There is an unmatched %c!\n", curr);
                return 1;
            }
            else
            {
                (*num)--;
            }
        }
        last = curr;
    }
    return 0;
}

void initializeStatus(struct ht_Table *table)
{
    ht_put(table, '\'', 0);
    ht_put(table, '"', 0);
    ht_put(table, '{', 0);
    ht_put(table, '[', 0);
    ht_put(table, '<', 0);
    ht_put(table, '(', 0);
}

void initializeOpposites(struct ht_Table *table)
{
    ht_put(table, '{', (int) '}');
    ht_put(table, '[', (int) ']');
    ht_put(table, '<', (int) '>');
    ht_put(table, '(', (int) ')');
    ht_put(table, '}', (int) '{');
    ht_put(table, ']', (int) '[');
    ht_put(table, '>', (int) '<');
    ht_put(table, ')', (int) '(');
}

void skipBlockComment(FILE *f)
{
    char last = 'a';
    char curr;
    while((curr = fgetc(f)) != '/' && last != '*')
    {
        last = curr;
    }
    skipLineComment(f);
    return;
}

void skipLineComment(FILE *f)
{
    char curr;
    while((curr = fgetc(f)) != '\n')
        ;
    return;
}

int skipString(FILE *f, char start)
{
    if(!(start == '\'' || start == '"'))
    {
        printf("You didn't give me a string, fool!\n");
        return -1;
    }
    char curr;
    int escaping = 0;
    while((curr = fgetc(f)) != EOF)
    {
        printf("%c", curr);
        if(curr == '\\' && !escaping)
        {
            escaping = 1;
            continue;
        }
        if(curr == start)
        {
            if(!escaping)
                return 0;
            else
                escaping = 0;
        }
        else if(curr == '\n')
        {
            if(!escaping)
            {
                return 1;
            }
            else
            {
                escaping = 0;
            }
        }
        if(escaping)
            escaping = 0;
    }
    // If we get down here, the file ended in the middle of a string
    return 1;
}

int search(char *haystack, int length, char needle)
{
    // Linear for now. 
    int i;
    for(i = 0; i < length; ++i)
    {
        if(haystack[i] == needle)
            return i;
    }
    return -1;
}

