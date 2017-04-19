using System;

using System.Collections.Generic;

using System.Linq;



namespace Kattis

{

  public class Program

  {

    public static void Main()

    {

        Dictionary<string,int> c = new Dictionary<string,int>();

        string s;

        int day = 0;

        while((s = Console.ReadLine()) != null)

        {

          string p;

          day++;

          while((p = Console.ReadLine()) != "CLOSE")

          {

            var ar = p.Split(' ');

            if(ar[0] == "ENTER")

            {

                if(c.ContainsKey(ar[1]))

                    c[ar[1]] -= int.Parse(ar[2]);

                else

                    c[ar[1]] = -int.Parse(ar[2]);

            }

            else

            {

                c[ar[1]] += int.Parse(ar[2]);

            }

              

          }

          Console.WriteLine("Day "+day);

          c.Select(x => string.Format("{0} ${1}",x.Key,(x.Value*0.1).ToString("F2"))).OrderBy(x => x).ToList().ForEach(x => Console.WriteLine(x));

          Console.WriteLine();

          c.Clear();

        }

    }

  }

}