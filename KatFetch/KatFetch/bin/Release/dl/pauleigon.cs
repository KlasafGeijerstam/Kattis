using System;

                    

public class Program

{

    public static void Main()

    {

        int n,p,q;

        var y = Console.ReadLine().Split(' ');

        n = int.Parse(y[0]);

        p = int.Parse(y[1]);

        q = int.Parse(y[2]);

        

        Console.WriteLine((p+q)/n % 2 == 0 ? "paul": "opponent");

        



    }

}