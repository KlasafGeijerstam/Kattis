using System;
using System.Linq;

public class Program
{
    public static void Main()
    {
        var ar = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToArray();
        Console.WriteLine(new DateTime(2009,ar[1],ar[0]).DayOfWeek);
    }

}
