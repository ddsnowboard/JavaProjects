// So I had to look this one up on the internet, although evidently
// one of the recursive ideas would have worked eventually. But this one 
// used dynamic programming, which is probably something I should know something about anyway. 
// What it does is says that we know that there is one way to make 0 coins (an idea that seems 
// ridiculous but if you think about it enough kinda makes sense. Regardless, much like 0!, it
// has to be that way for the algorithm to work simply.)
// Once we have 0, we can go to 1 once for each type of coin, subtract the value of that coin, and then 
// add on the amount of ways we can make that amount. Of course, for one, we can only use one coin, and that
// takes us to 0, which is 1. If we count up, we can guarantee that everything before the one we're on will
// be filled in, so we can just do it in one step per amount per denomination, which is pretty fast. 

// I guess the question is whether I could have come up with this on my own. I think my mindset was 
// into recursion when I was working on this, and my mind would have wandered over that way, but I think
// that I could have gotten it recusrively. And having done that, if I found that it was too slow, I bet 
// I could have refactored it to work with a lookup table or something. So I think I could have made it work
// one way or another by myself. 

// Separate from that, I used C++ for this, which was actually, if you look waaaay back in time, my first language, 
// but I haven't written a line of it in years, save for the 2 they had us write in CS 125, which I just deleted and wrote
// in C. I don't understand why people get so excited about C++. Linus Torvalds hates it with the fire of a thousand suns, but
// that's probably because he's smart enough to do everything in straight ANSI C, which I wish I could do but I don't think I can. 
// I also know people that love C++, like my 196 CA's, who called it "the one true language." Their whole pro-C++ feeling I don't understand, 
// but it can do everything. I do get that part. It does have so many features. You can program using nothing but templates, which is pretty
// amazing, but I think some would make the case that that's a bad thing. The appeal of so many languages is that they either are simple, like C, 
// or seem simple, like Python. C doesn't do anything. If you can understand some admittedly pretty strange ideas about memory management and pointers, 
// that's it. You know C. End of story. There just isn't that much. It's not a large language, to paraphrase, so it doesn't befit a large book. Python seems simple, 
// at least. You can do a lot with only a few parts of the language, and the other parts don't run around butting into everything. They just sit in the background, 
// and when you notice that you're doing something some annoying way all the time, they're right there to suggest you use a list comprehension or drop a lambda 
// function into `map` or something. They get you, or at least they got me, right when you best understand their usefulness, and they seem at the peak of
// utility and the imus of silliness. 

// I don't know why I didn't like C++ so much, but it bugged me. There are really two things about it that just got me: the error messages and the fact that 
// I just didn't want to keep writing in it. An early implementation of this used a linked list and I had to write my own templates and all this stuff, and 
// I had to write my own hashing function, which was a dreadful experience, and the error messages were from the intersection of templates, weird operator 
// overloads, and Hell. They were pages long and they were full of colons and angle brackets but nothing of real use. It took me forever to figure out where 
// they were coming from, and when I finally did, it didn't really help because I was fighing with the computer over something about constant pointers
// versus mutable objects. I didn't understand and it was a disaster. 
// The other thing is that when I was writing C++, I just wanted to not. Writing Python and C and even Java brings me joy, but not this. And it's probably 
// partially because I didn't know where they spatula was and the parking brake wasn't where I expected it to be, but I fiddled for a day with Ruby, and 
// I have never done anything with Ruby, I hadn't looked at it for about 2 years, but I opened vim and wrote some and it just knew what I wanted and did it. 
// It was amazing, I've never seen anything like it. I had no idea what I was doing and the computer just knew. C++ wasn't that. I guess the fact of the matter 
// is that people spend their whole careers learning every weird edge of C++ and that's why, because if you know all the edges, you can make an 8 bit microprocessor
// dance like nothing you've ever seen before with 10 lines of code, but I don't have a whole career behind C++ yet so it just makes me a little bit sad. 

// Next week: Rust! 

#include <cstdio>
#include <iostream>

#define NUMBER_OF_DENOMINATIONS 8
#define STARTING_AMOUNT 200
using namespace std;

void printArray(int arr[STARTING_AMOUNT + 1]);

int main(int argc, char** argv)
{
    const int DENOMINATIONS[NUMBER_OF_DENOMINATIONS] = {1, 2, 5, 10, 20, 50, 100, 200};
    int ways[STARTING_AMOUNT + 1];
    ways[0] = 1;
    for(int i = 1; i <= STARTING_AMOUNT; i++)
        ways[i] = 0;

    for(int i = 0; i < NUMBER_OF_DENOMINATIONS; i++)
    {
        int coinAmount = DENOMINATIONS[i];
        // cout << "coinAmount is " << coinAmount << "\n";
        for(int j = coinAmount; j <= STARTING_AMOUNT; j++)
        {
            if(j <= STARTING_AMOUNT && j - coinAmount >= 0)
                ways[j] += ways[j - coinAmount];
        }
        // printArray(ways);
    }
    cout << "The answer is " << ways[STARTING_AMOUNT] << "\n";
}

void printArray(int arr[STARTING_AMOUNT + 1])
{
    for(int i = 0; i <= STARTING_AMOUNT; i++)
        cout << arr[i] << "\n";
    cout << "\n\n\n\n";
}
