using System;
using System.Linq;

public class Program
{
    public static void Main()
    {
        var p = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToArray();
        var h = p[0];
        var a = p[1];
        
        if(a >= 0 && a <= 180){
            Console.WriteLine("safe");

        }
        else{
            Console.WriteLine((int)Math.Abs((h/Math.Sin(a*Math.PI/180))));
        }
        
    }
}
