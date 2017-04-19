using System;

using System.Linq;

using System.Globalization;

namespace Kattis

{

    class Program

    {

        static void Main(string[] args)

        {

            var i = int.Parse(Console.ReadLine());

            var s = Console.ReadLine();

            var s2 = Console.ReadLine();

            bool error = false;

            if(i % 2 == 0)

            {

                for (int j = 0; j < s.Length; j++)

                {

                    if (s[j] != s2[j])

                        error = true;

                }

            }

            else

            {

                for (int j = 0; j < s.Length; j++)

                {

                    if (s[j] == s2[j])

                        error = true;

                }

            }

            Console.WriteLine(error ? "Deletion failed" : "Deletion succeeded");

        }

    }

}