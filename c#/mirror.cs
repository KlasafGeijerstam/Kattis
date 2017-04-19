using System;

using System.Collections.Generic;

using System.Linq;



namespace bij

{

    class Program

    {

        static void Main(string[] args)

        {

            int c = int.Parse(Console.ReadLine());

            for (int i = 0; i < c; i++)

            {

                var ar = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToArray();

                int h = ar[0];

                int w = ar[1];

                var arr = new string[h];

                

                for(int y = 0; y < h; y++)

                {

                    arr[y] = Console.ReadLine();

                }

                Console.WriteLine("Test " + (i+1));

                for(int y = h-1; y >= 0; y--)

                {

                    for(int x = w-1; x >= 0; x--)

                    {

                        Console.Write(arr[y][x]);

                    }

                    Console.WriteLine();

                }



            }

        }

    }

}