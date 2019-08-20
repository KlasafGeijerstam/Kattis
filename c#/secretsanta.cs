using System;

namespace Kattis
{
    public class Program
    {
        public static void Main()
        {
            double[] arr = {0, 1, 0.5, 0.666667, 0.625, 0.633333, 0.631944, 0.632143, 0.632118, 0.632121 };
            var p = long.Parse(Console.ReadLine());
            if(p < 10)
                Console.WriteLine(arr[p]);
            else
                Console.WriteLine(arr[9]);
        }
    }
}
