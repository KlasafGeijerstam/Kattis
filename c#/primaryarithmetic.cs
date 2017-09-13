using System;
using System.Linq;

namespace Kattis
{
    class Program
    {
        public static void Main()
        {
            int[] m1, m2;

            while (true)
            {
                var m = Console.ReadLine().Split();
                if (m[0] == "0" && m[1] == "0")
                    return;
                m1 = m[0].PadLeft(12, '0').ToCharArray().Select(x => int.Parse(x.ToString())).ToArray();
                m2 = m[1].PadLeft(12, '0').ToCharArray().Select(x => int.Parse(x.ToString())).ToArray();

                int carry = 0;
                for (int i = 11; i > 0; i--)
                {
                    if(m2[i] + m1[i] > 9)
                    {
                        carry++;
                        m2[i] = (m2[i] + m1[i]) - 10;
                        m1[i-1]++;
                    }
                    else
                        m2[i] += m1[i];
                    
                }
                Console.WriteLine((carry == 0 ? "No carry " : $"{carry} carry ") + (carry > 1 ? "operations." : "operation."));
            }

        }
    }
}
