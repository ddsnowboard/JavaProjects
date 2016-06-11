#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#define FILENAME "words.txt"
#define MAXWORD 25
#define BASE_NUMBER 1000
#define DEFAULT_INCREASE 30

int isTriangle(char *word);
int isPrintableCharacter(char c);
float reverseTriangle(int i);
int addUpNumbers(char *word);
char **readFile(FILE *f);
void chomp(char *s);
int search(int *haystack, int length, int needle);

int main(int argc, char **argv)
{
    int numberOfWords = 0;

    FILE *f = fopen(FILENAME, "r");
    char **words = readFile(f);
    char *next;
    while((next = *words++)[0] != EOF)
    {
        chomp(next);
        if(isTriangle(next))
            numberOfWords++;
    }
    printf("There were %d words\n", numberOfWords);
    return 0;
}

int isTriangle(char *word)
{
    float reverse = reverseTriangle(addUpNumbers(word));
    return floorf(reverse) == reverse;
}

void chomp(char *line)
{
    // This stolen from my implementation of K&R exercise 1-21.
    int lastCharIndex = 0;
    int i = 0;
    char curr = line[i];
    do
    {
        if(isPrintableCharacter(curr))
            lastCharIndex = i;
    } while((curr = line[++i]));
    line[lastCharIndex + 1] = '\0';
}

int addUpNumbers(char *word)
{
    char c;
    int out = 0;
    while((c = *word++))
    {
        out += c - 'A' + 1;
    }
    return out;
}

float reverseTriangle(int i)
{
    return (sqrt(1 + 8 * i) -1) / 2;
}

int search(int *haystack, int length, int needle)
{
    // Implement a linear search first, then we can change
    while(length--)
    {
        if(needle  == *haystack++)
            return 1;
    }
    return -1;
}

int isPrintableCharacter(char c)
{
    return (c <= '~' && c >= '!');
}

char **readFile(FILE *f)
{
    int commas = 0;
    char c;
    while((c = fgetc(f)) != EOF)
        if(c == ',')
            commas++;
    // There are one more words than commas, plus one extra for an EOF
    char **output = malloc((commas + 2) * sizeof(char *));
    char **walker = output;
    char next;
    fseek(f, 0, SEEK_SET);
    char *currString;
    int i = 0;
    int inWord = 0;
    for(; i < commas + 1; ++i)
    {
        currString = malloc(MAXWORD);
        *walker++ = currString;
        while((next = fgetc(f)) != EOF)
        {
            if(next == ',')
                continue;
            if(next == '"')
            { 
                if(inWord)
                {
                    inWord = 0;
                    *currString++ = '\0';
                    break;
                }
                else
                {
                    inWord = 1;
                continue;
                }
            }
            *currString++ = next;
        }
    }
    *walker = malloc(2);
    (*walker)[0] = EOF;
    (*walker)[1] = '\0';
    return output;
}
