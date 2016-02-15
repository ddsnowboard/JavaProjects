#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int count(char* s, char* toFind)
{
    size_t len = strlen(s);
    size_t lenToFind = strlen(toFind);
    int count = 0;
    int i;
    for(i = 0;i < len; i++)
    {
        if(s[i] == toFind[0])
        {
            int j = 0;
            int found = 1;
            for(;j < lenToFind; j++)
            {
                if(s[i + j] != toFind[j])
                {
                    found = 0;
                }
            }
            if(found == 1)
            {
                count++;
            }
        }
    }
    return count;
}
int main(void) 
{
    char* CUE_LIST = "MasterCueList.txt";
    char* OUTPUT_FILE = "list.html";
    int MAX_LINE_LENGTH = 1000;
    char* TAB = "    ";
    FILE* in = fopen(CUE_LIST, "r");
    FILE* out = fopen(OUTPUT_FILE, "w");
    char* SCRIPTS = "<html><head><style>*{font-family: monospace; } p { font-size: 1.4em;} .clickable{margin-top:0;margin-bottom:0}.clicked{background-color:#000000;color:#ffffff;}span{text-decoration:underline;}</style><script src=\"https://code.jquery.com/jquery-1.11.2.min.js\"></script><script>$(document).ready(function(){var last=null;$(\".clickable\").click(function(){$(this).toggleClass(\"clicked\");if(last!=null){last.toggleClass(\"clicked\");}; last=$(this);});$(document).keydown(function(event){if(event.which===37){last.toggleClass(\"clicked\");$(\"#\"+(parseInt(last.attr('id'),10)-1).toString()).toggleClass(\"clicked\");last=$(\"#\"+(parseInt(last.attr('id'),10)-1).toString());}else if(event.which===39){last.toggleClass(\"clicked\");$(\"#\"+(parseInt(last.attr('id'),10)+1).toString()).toggleClass(\"clicked\");last=$(\"#\"+(parseInt(last.attr('id'),10)+1).toString());} console.log(\"#\"+(parseInt(last.attr('id'),10)+1).toString());});});</script><ul>";
    fprintf(out, "%s", SCRIPTS);
    char* line = malloc(MAX_LINE_LENGTH);
    int currLine = 0;
    int depth = 0;
    while(line = fgets(line, MAX_LINE_LENGTH, in))
    {
        int tabs = count(line, TAB);
        if(tabs > depth)
        {
            fprintf(out, "<ul>");
        }
        else if(tabs < depth)
        {
            int difference = depth - tabs;
            int i = 0;
            for(; i < difference; i++)
            {
                fprintf(out, "</ul></li>");
            }
        }
        fprintf(out, "<li><p id=\"%d\" class=\"clickable\">%s</p>", currLine, line);
        currLine++;
        if(tabs < depth)
        {
            int difference = tabs - depth;
            int i = 0;
            for(; i < difference; i++)
            {
                fprintf(out, "</ul>");
            }
        }
        depth = tabs;
    }
    fclose(out);
    fclose(in);
}
