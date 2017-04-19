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

                var god = new List<int>();

                

                var mec = new List<int>();

                Console.ReadLine();

                Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToList().ForEach(x => god.Add(x));

                Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToList().ForEach(x => mec.Add(x));

                god.OrderByDescending(x => x);

                mec.OrderByDescending(x => x);



                while (god.Count > 0 && mec.Count > 0)

                {

                    if (god[god.Count - 1] < mec[mec.Count - 1])

                        god.RemoveAt(god.Count - 1);

                    else if (mec[mec.Count - 1] < god[god.Count - 1])

                        mec.RemoveAt(mec.Count - 1);

                    else

                        mec.RemoveAt(mec.Count - 1);

                }



                if(god.Count > mec.Count)

                    Console.WriteLine("Godzilla");

                else if(mec.Count > god.Count)

                    Console.WriteLine("MechaGodzilla");

                else

                    Console.WriteLine("uncertain");

            } 

        }



    }

}