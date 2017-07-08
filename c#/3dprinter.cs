using System;
namespace Kattis
{
    public class Program
    {
        public static void Main() => Console.WriteLine((int)(Math.Ceiling(Math.Log10(int.Parse(Console.ReadLine()))/Math.Log10(2))+1));
    }
}
