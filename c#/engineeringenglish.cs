using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace Kattis
{
    public class Program
    {
        public static void Main()
        {
            var lst = new HashSet<string>();
            string s;
            var sb = new StringBuilder();
            while ((s = Console.ReadLine()) != null)
            {
                var p = s.Split(' ');

                bool frst = true;
                foreach (var ss in p)
                {
                    if (!lst.Contains(ss.ToLower()))
                    {
                        lst.Add(ss.ToLower());
                        if (!frst)
                            sb.Append(" " + ss);
                        else
                            sb.Append(ss);
                    }
                    else
                    {
                        if (!frst)
                            sb.Append(" .");
                        else
                            sb.Append(".");
                    }
                    frst = false;
                }
                sb.Append("\n");
            }
            Console.WriteLine(sb);
        }
    }
}
