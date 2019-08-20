using System;
using System.Linq;
                    
public class Program
{
    public static void Main()
    {
        int x = int.Parse(Console.ReadLine());
        int tot = x;
        Enumerable.Range(1,int.Parse(Console.ReadLine())).ToList().ForEach(p =>  tot += x -int.Parse(Console.ReadLine()));
        Console.WriteLine(tot);
    }
}
