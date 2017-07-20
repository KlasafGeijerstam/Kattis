using System;
using System.Linq;

namespace Kattis
{
  public class Program
  {
    public static void Main()
    {
        int[] p = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToArray();
        double tot = Math.PI * Math.Pow(p[0],2);
        Console.WriteLine(100*(1-((tot - (Math.Pow(p[0]-p[1],2))*Math.PI)/tot)));
    }
  }
}
