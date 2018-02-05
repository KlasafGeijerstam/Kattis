using System;
using System.Linq;

public class Program
{
	public static void Main()
	{
		var p = Console.ReadLine().Split().Select(x => int.Parse(x)).ToList();
		p.Sort();
		Console.WriteLine(p[1] - p[0] == p[2] - p[1] ? p[2] + p[1] - p[0] : (p[1] - p[0] > p[2] - p[1] ? p[0] + p[2] - p[1] : p[1] + p[1] - p[0]));
	}
}
