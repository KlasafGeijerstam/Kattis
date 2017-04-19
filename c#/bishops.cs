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

            string str;

            while((str = Console.ReadLine()) != null)

            {

                var i = int.Parse(str);

                if(i == 1)

                    Console.WriteLine(1);

                else if(i > 1)

                    Console.WriteLine(i + i-2);

                else

                    Console.WriteLine(0);

            }

        }

    }

}