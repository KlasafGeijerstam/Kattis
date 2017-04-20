using System;
using System.Linq;

public class Program
{
    public static void Main()
    {
        string s;
        var cc = 1;
        while((s = Console.ReadLine()) != null){
            var ar = (s + " " + Console.ReadLine()).Split(' ').Select(x =>int.Parse(x)).ToArray();
            double c = 1/((ar[0]*ar[3])-(ar[1]*ar[2])); 
            Console.WriteLine("Case " + cc++ +":"+'\n' + c*ar[3] + " " + c*(-ar[1]) + '\n' + c*(-ar[2]) + " " + c*ar[0]);
            Console.ReadLine();
        }
    }
}
