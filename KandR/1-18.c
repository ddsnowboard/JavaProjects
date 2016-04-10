#include <stdio.h>
#include <stdlib.h>

int main(int argc, char** argv)
{
    size_t SIZE = 0;
    char *lineHolder = NULL;
    while(getline(&lineHolder, &SIZE, stdin) != -1)
    {
        if(lineHolder[0] == '\n')
            continue;
        char* lastChar;
        char* line = lineHolder;
        do
            if(!(*line == ' ' || *line == '\t' || *line == '\n'))
                lastChar = line;
        while(*(++line));
        *(lastChar + 1) = '\0';
        printf("%s\n", lineHolder);
    }
    return 0;
}
