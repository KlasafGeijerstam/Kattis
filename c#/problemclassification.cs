using System;
using System.Collections.Generic;

namespace Csharp
{
    class ProblemType
    {
        public string name;
        public int count;

        public ProblemType(string name)
        {
            this.name = name;
        }

        public override string ToString()
        {
            return name;
        }
    }

    class Program
    {
        static void Main(string[] args)
        {
            var cat = int.Parse(Console.ReadLine());
            var dic = new Dictionary<string, List<ProblemType>>();
            var types = new List<ProblemType>();
            string line;

            for (int i = 0; i < cat; i++)
            {
                var s = Console.ReadLine().Split();
                var type = new ProblemType(s[0]);
                types.Add(type);
                for (int j = 2; j < s.Length; j++)
                {
                    if (!dic.ContainsKey(s[j]))
                        dic[s[j]] = new List<ProblemType>();
                    dic[s[j]].Add(type);
                }
            }

            while ((line = Console.ReadLine()) != null)
                foreach (var w in line.Split())
                    if (dic.ContainsKey(w))
                        dic[w].ForEach(x => x.count++);

            types.Sort((x, y) =>
            {
                if (x.count != y.count)
                    return y.count.CompareTo(x.count);
                return x.name.CompareTo(y.name);
            });
            int high = 0;
            foreach (var item in types)
            {
                if (item.count < high)
                    break;
                Console.WriteLine(item);
                high = Math.Max(item.count, high);
            }
        }
    }
}
