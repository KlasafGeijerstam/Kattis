using System;

namespace bij
{
    class Program
    {

        static void Main(string[] args)
        {

            int c = int.Parse(Console.ReadLine());
            var sarr = new string[] { "simon says " };
            for (int i = 0; i < c; i++)
            {
                var s = Console.ReadLine();
                if(s.Length > 11)
                    if(s.Substring(0,11) == "simon says ")
                        Console.WriteLine(s.Substring(11,s.Length-11));
                    else
                        Console.WriteLine();
            }
            
        }
    }
    
}
