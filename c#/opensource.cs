using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Numerics;
using System.Text;
using System.Threading;

namespace Kattis
{
    public class Program
    {
        public static void Main()
        {
            while (true)
            {
                var p = new Dictionary<string, HashSet<string>>();
                var u = new Dictionary<string, int>();

                string x, c = "";
                while((x = Console.ReadLine()) != "1")
                {
                    if (x == "0")
                        return;
                    if (char.IsUpper(x[0]))
                    {
                        c = x;
                        if (!p.ContainsKey(c))
                            p.Add(c, new HashSet<string>());
                    }
                    else
                    {
                        if (!u.ContainsKey(x))
                            u.Add(x, 0);
                        if (p[c].Contains(x))
                            u[x]--;
                        p[c].Add(x);
                        u[x]++;
                    }
                }
                var mult = u.Where(us => us.Value > 1);
                foreach (var pr in p)
                    foreach (var m in mult)
                        if (pr.Value.Contains(m.Key))
                            pr.Value.Remove(m.Key);

                var ot = p.Select(i => new Tuple<string,int>(i.Key, i.Value.Count)).OrderByDescending(i => i,Comparer<Tuple<string,int>>.Create((i,j) => i.Item2.CompareTo(j.Item2) == 0 ? i.Item1.CompareTo(j.Item1)*-1 : i.Item2.CompareTo(j.Item2)));
                foreach(var pr in ot)
                    Console.WriteLine($"{pr.Item1} {pr.Item2}");
            }
        }
    }
}
