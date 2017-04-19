using System;

using System.Security.Cryptography;

using System.Text;

using System.Linq;



public class Program

{

    public static void Main()

    {

        string s2;

        while((s2 = Console.ReadLine()) != null){

            

            var s = s2.Split(' ').ToList().Select(x => long.Parse(x)).ToList();

            Console.WriteLine(Math.Abs(s[0]-s[1]));

            

        }

        

    }   

}