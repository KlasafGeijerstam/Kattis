using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace Kattis
{
    class Program
    {
        
        static void Main(string[] args)
        {
            int ballp = 1;
            foreach (var c in Console.ReadLine())
            {
                if(ballp == 1)
                {
                    if (c == 'A')
                        ballp = 2;
                    else if (c == 'C')
                        ballp = 3;
                }
                else if(ballp == 2)
                {
                    if (c == 'B')
                        ballp = 3;
                    else if (c == 'A')
                        ballp = 1;
                }
                else if(ballp == 3)
                {
                    if (c == 'B')
                        ballp = 2;
                    else if (c == 'C')
                        ballp = 1;
                }
            }
            Console.WriteLine(ballp);
        }
    }
}
  
    
