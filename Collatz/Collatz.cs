using System;
using System.Threading;
namespace HelloWorld
{
    class Info {
        public ulong i;

        public Info(ulong i) {
            this.i = i;
        }
    }

    class Hello 
    {
        static void Main() 
        {
            const int kMax = 1000000;
            for(var i = 1; i < kMax; i++) {
                var info = new Info((ulong) i);
                ThreadPool.QueueUserWorkItem(Collatz, info);
            }
        }

        // Returns 0 if Collatz Conjecture works, a number if otherwise.
        static void Collatz(Object oInfo)
        {
            var info = (Info) oInfo;
            var originali = info.i;
            var i = info.i;
            if(i == 0)
                return;
            // These are to make sure that we aren't just going in a circle.
            ulong newest = 0;
            ulong secondNewest = 0;
            while(i != 1)
            {
                if(i % 2 == 0)
                {
                    i /= 2;
                }
                else if(i % 2 != 0)
                {
                    i = 3 * i + 1;
                }
                else
                {
                    Console.WriteLine("Something really bad just happened...");
                    return;
                }
                if(i == newest || i == secondNewest)
                {
                    Console.WriteLine("Doesn't work for %d", originali);
                    return;
                }
                else 
                {
                    secondNewest = newest;
                    newest = i;
                }
            }
            return;
        }
    }
}
