using System;

                    

public class Program

{

    public static void Main()

    {

        int i = int.Parse(Console.ReadLine());

            for(int j = 0; j < i; j++){

                Console.WriteLine(Math.Ceiling((int.Parse(Console.ReadLine()))/400f));  

            }

    }

}