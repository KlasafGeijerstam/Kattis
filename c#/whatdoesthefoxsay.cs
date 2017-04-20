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
            int cases = int.Parse(Console.ReadLine());
            for (int i = 0; i < cases; i++)
            {
                var s1 = Console.ReadLine().Split(' ').ToList();
                List<string> ss = new List<string>();
                string s;
                while ((s = Console.ReadLine())!= "what does the fox say?")
                {
                    ss.Add(s.Split(' ')[2]);
                }
                s = "";
                foreach(var ap in ss){
                    if(s1.Contains(ap))
                        s1.RemoveAll(x => x == ap);
                }
                foreach(var ass in s1)
                    Console.Write(ass + " ");
                 
                Console.WriteLine();
            }
        }
    }
}
  
    
