using System;


namespace bij
{
    class Program
    {
        static void Main(string[] args)
        {
            int c = int.Parse(Console.ReadLine());
            double sum = 0;

            for (int i = 0; i < c; i++)
            {
                var s = Console.ReadLine();
                int r = int.Parse(s.Substring(0, s.Length - 1));
                int p = int.Parse(s[s.Length - 1].ToString());
                sum += Math.Pow(r,p);
            }
            Console.WriteLine(sum);
        }
    }
}
