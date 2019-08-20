using System;
using System.Linq;

public class Program
{
    public static void Main()
    {
        Console.ReadLine();
        var l = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).OrderByDescending(x => x).ToList();
        var discount = 0l;
        for(int i = 2; i < l.Count; i+=3)
            discount += l[i];
        Console.WriteLine(discount);
    }
}
