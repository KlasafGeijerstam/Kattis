using System;
using System.Linq;

namespace Kattis
{
    public class Program
    {
        public static void Main()
        {
            string s;
            while((s = Console.ReadLine()) != null)
            {
                int inc = int.Parse(s.Split()[1]);
                int next = 0;
                char prev;
                var mod = s.Split()[0].Reverse().ToList();

                for (int i = 0; i < mod.Count; i++)
                {
                    int mem = char.IsNumber(mod[i]) ? 10 : 26;
                    char a = char.IsNumber(mod[i]) ? '0' : char.IsUpper(mod[i]) ? 'A' : 'a';
                    int tod;
                    inc = Math.DivRem(inc, mem, out tod);
                    next = Math.DivRem(mod[i] - a + tod + next, mem, out tod);
                    
                    mod[i] = (char)(tod + a);

                    prev = mod[i];
                }

                inc += next;

                while (inc != 0)
                {
                    int mem = char.IsNumber(mod[mod.Count - 1]) ? 10 : 26;
                    char a = char.IsNumber(mod[mod.Count - 1]) ? '0' : (char.IsUpper(mod[mod.Count - 1]) ? 'A' : 'a');
                    int tot;
                    inc = Math.DivRem(inc - (mem == 10 ? 0 : 1), mem, out tot);
                    mod.Add((char)(tot + a));
                }

                mod.Reverse();
                Console.WriteLine(new string(mod.ToArray()));
            }
        }
    }
} 
