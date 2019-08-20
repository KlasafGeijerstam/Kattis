using System;

using System.Collections.Generic;

using System.Globalization;

using System.Linq;



namespace Kattis

{

    public class Program

    {

        public static void Main()

        {

            while (true)

            {

                var l = new Dictionary<Vector2, bool>();

                var p = Console.ReadLine().Split();

                var d = double.Parse(p[0], CultureInfo.InvariantCulture);

                var a = int.Parse(p[1]);

                if (d == 0.0f && a == 0)

                    break;

                for (int i = 0; i < a; i++)

                {

                    var s = Console.ReadLine().Split();

                    l.Add(new Vector2(double.Parse(s[0]), double.Parse(s[1])), false);

                }

                var papp = l.ToDictionary((x => x.Key),x => x.Value);

                foreach(var x1 in l)

                {

                    foreach(var x2 in l)

                    {

                        if (x1.Key == x2.Key)

                            continue;

                        if (Vector2.Distance(x1.Key, x2.Key) <= d)

                            papp[x1.Key] =  papp[x2.Key] = true;

                    }

                }

                Console.WriteLine($"{papp.Count(x => x.Value)} sour, {papp.Count(x => !x.Value)} sweet");

            }

        }

    }

    class Vector2

    {

        public double x, y;

        public static double Distance(Vector2 x,Vector2 y)

        {

            return Math.Sqrt(Math.Pow(x.x - y.x, 2) + Math.Pow(x.y - y.y, 2));

        }

        public Vector2(double x,double y)

        {

            this.x = x;

            this.y = y;

        }

        public override int GetHashCode()

        {

            return x.GetHashCode()*y.GetHashCode()*7;

        }

    }

} 