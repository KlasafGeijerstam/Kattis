using System;



namespace Kattis

{

  public class Program

  {

    public static void Main()

    {

        string s;

        while((s = Console.ReadLine()) != null)

        {

            Console.WriteLine(s.IndexOf("problem",StringComparison.OrdinalIgnoreCase) >= 0 ? "yes" : "no");

        }

        

    }

  }

}