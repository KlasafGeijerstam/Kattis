using System;
using System.Diagnostics;
using System.Collections.Generic;
using System.IO;
using System.Numerics;

public class Program
{
    public static void Main()
    {
        int c = int.Parse(Console.ReadLine());
        for(int i = 0; i < c; i++)
        {
            var s1 = Console.ReadLine();
            var s2 = Console.ReadLine();
            Console.WriteLine(s1);
            Console.WriteLine(s2);
            for(int j = 0; j < s1.Length; j++)
                Console.Write((s1[j]==s2[j]) ? "." : "*");
            Console.Write("\n\n");
        }
        
    }
    
}
