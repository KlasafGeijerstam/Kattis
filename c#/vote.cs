using System;
using System.Collections.Generic;
using System.Linq;

namespace bij
{
    class Program
    {

        struct cand : IComparable
        {
            public int vot;
            public int num;

            public cand(int v,int n)
            {
                vot = v;
                num = n;
            }
            public int CompareTo(object obj)
            {
                return vot.CompareTo(obj);
            }
        }

        static void Main(string[] args)
        {
            int count = int.Parse(Console.ReadLine());
            for (int i = 0; i < count; i++)
            {
                var arr = new cand[int.Parse(Console.ReadLine())];
                var total = 0;
                for (int c = 0; c < arr.Length; c++)
                {
                    arr[c] = new cand(int.Parse(Console.ReadLine()),c+1);
                    total += arr[c].vot;
                }
                arr = arr.OrderByDescending(x => x.vot).ToArray();
                if(arr[0].vot > arr[1].vot)
                {
                    Console.WriteLine(((arr[0].vot > total/2) ? "majority winner " : "minority winner ") + arr[0].num);
                }
                else
                    Console.WriteLine("no winner");
            }
        }
    }
}
