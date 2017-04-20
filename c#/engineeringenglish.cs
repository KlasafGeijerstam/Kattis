using System;
using System.Collections.Generic;
using System.Linq;

namespace Kattis
{
  public class Program
  {
    public static void Main()
    {
        List<string> lst = new List<string>();
        
        string s;
        while((s = Console.ReadLine()) != null)
        {
            var p = s.Split(' ');
            
            bool frst = true;
            foreach(var ss in p)
            {
                if(!lst.Contains(ss.ToLower()))
                {
                    lst.Add(ss.ToLower());
                    if(!frst)
                        Console.Write(" " +ss);
                    else
                        Console.Write(ss);
                }
                else
                {
                    if(!frst)
                        Console.Write(" .");
                    else
                        Console.Write(".");
                }
                frst = false;
            }
            Console.Write("\n");
        }
        
    }
  }
}
