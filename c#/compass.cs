using System;



namespace Kattis

{

    class Program

    {

        static void Main(string[] args)

        {

            int n1 = int.Parse(Console.ReadLine());

            int n2 = int.Parse(Console.ReadLine());

            n2 -= n1;

            if (n2 > 180)

                n2 -= 360;

            if (n2 <= -180)

                n2 += 360;

            Console.WriteLine(n2);

        }

    }

}