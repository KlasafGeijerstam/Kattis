using System;

using System.Collections.Generic;

using System.Globalization;

using System.Linq;

using System.Text;

using System.Numerics;

using System.IO;



namespace Kattis

{

    public class Program

    {

        public static void Main()

        {

            int h;

            var b = false;

            var w = new StringWriter();

            var ot = Console.Out;

            Console.SetOut(w);

            w.NewLine = "\n";

            while((h = int.Parse(Console.ReadLine())) != 0)

            {

                if (!b)

                    b = true;

                else

                    Console.WriteLine("\n");



                var l = new List<String>();

                for (int i = 0; i < h; i++)

                    l.Add(Console.ReadLine());

                var m = l.Max(x => x.Length);



                for (int i = 0; i < l.Count; i++)

                {

                    for (int j = i+1; j < l.Count; j++)

                    {

                        l[j] = l[j].PadRight(l[i].Length);

                    }    

                }



                for (int i = 0; i < m; i++)

                {

                    for (int j = l.Count-1; j >= 0; j--)

                    {

                        if (i < l[j].Length)

                            Console.Write((l[j][i] == '-' ? '|' : l[j][i] == '|' ? '-' : l[j][i]));

                    }

                    if (i != m - 1)

                        Console.WriteLine();

                }

            }

            w.Flush();

            var sap = w.ToString().Split('\n').ToList();

            Console.SetOut(ot);

            foreach (var s in sap)

            {

                    Console.WriteLine(s.TrimEnd(' ', '\r','\n'));

            }

        }

    }

} 