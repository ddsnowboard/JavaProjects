#include <stdio.h>
#include <stdlib.h>

// Write a program to remove all comments from a C program. Don't forget to handle quoted strings
// and character constants properly. C comments do not nest.

int main(int argc, char **argv)
{
    char *line = NULL;
    size_t length = 0;
    int inBlock = 0;
    while(getline(&line, &length, stdin) != -1)
    {
        int i = 0;
        char curr = line[i];
        char lastChar = '\0';
        int commentable = 1;
        char beginningDelimiter;
        do{
            if(!(inBlock == 1))
            {
                if(curr == '/' && lastChar == '/' && commentable == 1)
                {
                    // We know that i - 1 is the first /, so we remove it.
                    line[i - 1] = '\0';
                    break;
                }
                else if(curr == '*' && lastChar == '/')
                {
                    inBlock = 1;
                }
                else if(curr == '\'' || curr == '"')
                {
                    if(!(lastChar == '\\'))
                    {
                        if(commentable)
                        {
                            commentable = 0;
                            beginningDelimiter = curr;
                        }
                        else
                        {
                            if(beginningDelimiter == curr)
                                commentable = 1;
                        }
                    }
                }
            }
            else
            {
                if(curr == '/' && lastChar == '*')
                    inBlock = 0;
                else
                    line[0] = '\0';
            }
            lastChar = curr;
        } while((curr = line[++i]));
        printf("%s", line);
    }
    return 0;
}
