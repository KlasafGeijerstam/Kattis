using System;
using System.Linq;
class Program 
{
    static void Main(string[] args)
    {
        Console.ReadLine ();
        var l = Console.ReadLine ().Split ().Select (int.Parse).ToList();
        var max = 0;
        var i = 0;
        for(var u = 0; u < l.Count; u++)
            if(l[u] > max && l.Count(j => j == l[u]) == 1){
                max = l[u];
                i = u + 1;
            }
        System.Console.WriteLine (i == 0 ? "none" : i.ToString());
    }
}
