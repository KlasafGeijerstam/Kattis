using System;



namespace Kattis

{

    class Program

    {

        public static void Main(string[] args)

        {





            var big = Console.ReadLine();

            if(big.Length < 10)

            {

                long c = 1;

                long num = long.Parse(big);

                while (true)

                {

                    num /= c++;

                    if (num == 1)

                    {

                        Console.WriteLine(c - 1);

                        return;

                    }

                        

                }

            }

            

            ulong i = 0;

            double a = 1;

            int len = big.Length;

            while (++i > 0)

            {

                a += Math.Log10(i);

                if(Math.Floor(a) == len)

                {

                    Console.WriteLine(i);

                    return;

                }

            }

        }

    }

}