using System;

using System.Linq;

                    

public class Program

{

    public static void Main()

    {

        var c = int.Parse(Console.ReadLine());

        for(int i = 0; i < c; i++){

            Console.ReadLine();

            var p = Console.ReadLine().Split(' ').Select(x => int.Parse(x));

            Console.WriteLine((p.Max()-p.Min())*2);

        }

    }

}