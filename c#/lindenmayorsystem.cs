using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace Csharp
{
    class Program
    {
        static void Main(string[] args)
        {
            var l = Console.ReadLine().Split().Select(int.Parse).ToArray();
            int n = l[0], m = l[1];
            
            var trans = new Dictionary<char, string>();
            for(var i = 0; i < n; i++)
            {
                var y = Console.ReadLine().Split(' ');
                trans[y[0][0]] = y[2];
            }
            
            var s = new StringBuilder(Console.ReadLine());
            for(var i = 0; i < m; i++)
            {
                var s2 = new StringBuilder();
                for (int j = 0; j < s.Length; j++)
                    s2.Append(trans.ContainsKey(s[j]) ? trans[s[j]] : s[j].ToString());
                s = new StringBuilder(s2.ToString());
            }
            Console.WriteLine(s);
        }
    }
}
