using System;

using System.Collections.Generic;

using System.Globalization;

using System.Linq;

using System.Numerics;



namespace Kattis

{

    public class Program

    {

        public static void Main()

        {

            foreach(var i in Enumerable.Range(0, int.Parse(Console.ReadLine())))

            {

                var x = Console.ReadLine().Split().Select(o => float.Parse(o,CultureInfo.InvariantCulture)).ToArray();

                var t = x[2] / (Math.Cos(DTR(x[1]))*x[0]);

                var y = x[0] * t * Math.Sin(DTR(x[1])) - 0.5 * 9.81f * t * t;

                Console.WriteLine((y < x[4]-1 && y > x[3]+1)? "Safe" : "Not Safe");

            }

        }

        static double DTR(double angle) =>  Math.PI* angle / 180.0;

    }

} 