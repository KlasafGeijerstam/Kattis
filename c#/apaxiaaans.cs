using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Katio
{
    class Program
    {
        static void Main(string[] args)
        {
            

                var s = Console.ReadLine();
                var tot = "";
                for (int i = 0; i < s.Length;)
                {
                    tot += s[i];
                    char lst = s[i];
                    i++;
                    if (i >= s.Length)
                        break;
                    while (i < s.Length && s[i] == lst)
                        i++;
                }
                Console.WriteLine(tot);
            
        }
    }
}