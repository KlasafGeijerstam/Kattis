using System;

namespace Csharp
{
    class Program
    {
        static void Main(string[] args)
        {
            var p = int.Parse(Console.ReadLine());
            for (int i = 0; i < p; i++)
            {
                var x = Console.ReadLine();
                if(x == "P=NP")
                    Console.WriteLine("skipped");
                else
                    Console.WriteLine(int.Parse((x.Split('+')[0])) + int.Parse((x.Split('+')[1])));
            }
        }
    }
}
