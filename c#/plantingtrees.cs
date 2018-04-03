using System;
using System.Linq;

public class Program 
{
    public static void Main() 
    {
        Console.ReadLine();
        var l = Console.ReadLine().Split().Select(int.Parse).OrderByDescending(x => x).ToList();
        var max = l[0];
        for(var i = 1; i < l.Count(); i++)
            max = Math.Max(--max,l[i]);
        Console.WriteLine(l.Count() + max + 1);
    }       
}
