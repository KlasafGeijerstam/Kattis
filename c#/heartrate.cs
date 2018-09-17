using System;
using System.Linq;

public class Program
{
    public static void Main()
    {
        Enumerable.Range(0, int.Parse(Console.ReadLine())).All(x => {
            var d = Console.ReadLine().Split().Select(double.Parse).ToArray();
            Console.WriteLine("{0:f5} {1:f5} {2:f5}", ((d[0] - 1)*60)/d[1], ((d[0])*60)/d[1], ((d[0] + 1)*60)/d[1]);
            return true;
        });
    }
}
