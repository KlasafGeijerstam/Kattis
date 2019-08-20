using System;

using System.Collections.Generic;

using System.Globalization;

using System.Linq;

using System.Text;



namespace Kattis

{

    public class Program

    {

        static StringBuilder tb = new StringBuilder();

        static Random r = new Random();

        public static void Main()

        {

            var k = Console.ReadLine().Split().Select(x => int.Parse(x)).ToArray();

            var b = new StringBuilder();

            

            for (int i = 0; i < k[0]; i++)

            {

                b.Append(NextString());

            }

            for (int i = 0; i < (k[1]/2)-k[0]+1; i++)

            {

                b.Append(NextString());

            }

            Console.WriteLine(b.ToString());

            Console.ReadLine();

        }

        static string NextString()

        {

            tb.Clear();

            for (int i = 0; i < 7; i++)

            {

                tb.Append((char)r.Next('a', 'z' + 1));

            }

            tb.Append(" ");

            return tb.ToString();

        }

    }

} 