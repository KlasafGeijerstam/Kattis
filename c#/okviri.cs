using System;

using System.Linq;



namespace Kattis

{

    public class Program

    {

        public static void Main()

        {

            var v = Console.ReadLine();

            var lst = new string[5];

            var l = false;

            for (int i = 1; i < v.Length+1; i++)

            {

                var c = i % 3 == 0 ? '*' : '#';

                lst[0] += $"..{c}.";

                lst[1] += $".{c}.{c}";

                lst[2] += l ? $"*.{v[i - 1]}." : $"{c}.{v[i-1]}.";

                lst[3] += $".{c}.{c}";

                lst[4] += $"..{c}.";

                l = c == '*';

            }

            var k = l ? '*' : '#';

            lst[0] += ".";

            lst[1] += ".";

            lst[2] += k;

            lst[3] += ".";

            lst[4] += ".";

            lst.ToList().ForEach(x => Console.WriteLine(x));

        }

    }

}