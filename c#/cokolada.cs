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

            var l = uint.Parse(Console.ReadLine());

            if(Closest(l) == l)

                Console.WriteLine(l + " " + 0);

            else

            {

                int breaks = 1;

                uint target = Closest(l) - l;

                uint cur = Closest(l) / 2;

                while(cur != target)

                {

                    breaks++;

                    if (cur / 2 < target)

                        target -= cur / 2;

                    cur = cur / 2;

                }

                Console.WriteLine(Closest(l) + " " + breaks);

            }

            Console.ReadLine();

        }



        static uint Closest(uint p)

        {

            if (Convert.ToString(p, 2).Count(x => x == '1') > 1)

            {

                return 2147483648 >> (Convert.ToString(p, 2).IndexOf('1') + 31 - Convert.ToString(p, 2).Length);

            }

            else

                return p;

        }

    }

}