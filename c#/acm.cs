using System;
using System.Linq;
using System.Collections.Generic;
using static System.Console;

class Program {
    public static void Main(String[] args) {
        var lines = new List<string>();       
        var ln = "";
        while((ln = ReadLine()) != null) {
            lines.Add(ln); 
        }
        lines.RemoveAt(lines.Count - 1);

        var sc = new Dictionary<string, int>(); 
        int tt = 0, ts = 0;
        foreach(var l in lines.Select(x => x.Split(" "))) {
            if(!sc.ContainsKey(l[1])) {
                sc[l[1]] = 0;
            }

            switch(l[2]) {
                case "right":
                    tt += sc[l[1]] * 20 + int.Parse(l[0]);
                    ts++;
                    break;
                case "wrong":
                    sc[l[1]]++;
                    break;
            }
        }

        WriteLine($"{ts} {tt}");
    }
}
