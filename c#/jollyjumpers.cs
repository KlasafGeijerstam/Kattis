using System;
using System.Collections.Generic;
using System.Linq;

class Program {
    static void Main() {
        string s;
        while((s = Console.ReadLine()) != null) {
            var p = s.Split().Select(int.Parse).ToList();
            p.RemoveAt(0);
            var l = new HashSet<int>(Enumerable.Range(1,p.Count-1));
            for(var i = 0; i < p.Count -1; i++)
                if(l.Contains(Math.Abs(p[i]-p[i+1])))
                    l.Remove(Math.Abs(p[i]-p[i+1]));
            System.Console.WriteLine(l.Count == 0 ? "Jolly" : "Not jolly");
        }
    }
}
