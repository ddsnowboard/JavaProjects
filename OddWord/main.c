#include<stdio.h>
#include<ctype.h>
char reverseNextWord() {
    char c = getchar();
    if(ispunct(c)) {
        return c;
    } else {
        char out = reverseNextWord();
        putchar(c);
        return out;
    }
}

int main(int argc, char** argv) {
    int wordsSeen = 0;
    char curr;
    while((curr = getchar()) != '.') {
        if(ispunct(curr))  {
            wordsSeen++;
            putchar(curr);
            if(wordsSeen % 2 == 1) {
                char c = reverseNextWord();
                putchar(c);
                if(c == '.')
                    return 0;
                wordsSeen++;
            }
        } else {
            putchar(curr);
        }  
    }
    putchar('.');
}
