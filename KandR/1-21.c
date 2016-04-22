#define MAXLEN 80
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

// Wirte a program to "fold" long input lines into two or more shorter lines after the last non-blank
// character that occurs before the n-th column of input. Make sure your program does something intelligent with very long lines, 
// and if there are no blanks or tabs before the specified column.

void fold(char **line);
void insert(char *s, int idx, char toInsert, int times);
void unshift(char *s, int idx);
void _fold(char **origPointer, char **line);
void chomp(char *line);
int isPrintableCharacter(char c);

int main(int argc, char **argv)
{
    char *line = NULL;
    size_t size = 0;
    while(getline(&line, &size, stdin) != -1)
    {
        chomp(line);
        fold(&line);
        printf("%s\n", line);
        line = NULL;
        size = 0;
    }
    return 0;
}

void fold(char **line)
{
    _fold(line, line);
}

void _fold(char **origPointer, char **line)
{
    int lastBreakableIndex = 0;
    char *s = *line;
    int i = 0;
    char curr = s[i];
    do
    {
        if(i >= MAXLEN)
        {
            if(lastBreakableIndex == 0)
            {
                (*origPointer) = realloc(*origPointer, strlen(*origPointer) + 3);
                lastBreakableIndex = MAXLEN;
                insert(s, lastBreakableIndex + 1, '\n', 1);
                insert(s, lastBreakableIndex + 1, '-', 1);
            }
            else
            {
                s[lastBreakableIndex] = '\n';
            }
            char *newString = &(s[lastBreakableIndex + 1]);
            _fold(origPointer, &newString);
            break;
        }
        else if(!isPrintableCharacter(curr))
            lastBreakableIndex = i;
    } while((curr = s[++i]));
}

void insert(char *s, int idx, char toInsert, int times)
{
    if(times == 0)
        return;
    char last = s[idx];
    int index = idx;
    s[idx] = toInsert;
    ++idx;
    char temp;
    do
    {
        temp = s[idx];
        s[idx] = last;
        last = temp;
    } while(s[idx++] != '\0');
    insert(s, index, toInsert, --times);
}

void chomp(char *line)
{
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

int isPrintableCharacter(char c)
{
    return (c <= '~' && c >= '!');
}

void unshift(char *s, int idx)
{
    do
    {
        s[idx] = s[idx + 1];
    } while(s[++idx]);
}
