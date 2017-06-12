using System;

using System.Linq;

using System.Numerics;

using System.Collections.Generic;

using System.IO;

using System.Diagnostics;



namespace Kattis

{



    public class Program

    {

        public static void Main()

        {

            int cases = int.Parse(Console.ReadLine());



            for (int i = 0; i < cases; i++)

            {

                Console.ReadLine();

                

                Console.ReadLine();

                var god = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).Max();

                var mec = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).Max();



                if(god >= mec)

                    Console.WriteLine("Godzilla");

                else

                    Console.WriteLine("MechaGodzilla");

                

            } 

        }



    }

}