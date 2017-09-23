using System;

namespace Kattis
{
    public class Program
    {
        public static void Main()
        {
            int cases = int.Parse(Console.ReadLine()), won = 0;
            for (int i = 0; i < cases; i++)
                won += Console.ReadLine().IndexOf("CD") >= 0 ? 0 : 1;
            Console.WriteLine(won);
        }
    }
}
