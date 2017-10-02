using System;
using System.Collections.Generic;
//Ugly..
namespace Kattis
{
    class Program
    {
        static void Main(string[] args)
        {
            while(true) 
            {
                int p = int.Parse(Console.ReadLine());
                if(p == 0)
                    break;

                var x = new List<double>();
                var y = new List<double>();
                for (int i = 0; i < p; i++)
                {
                    var r = Console.ReadLine().Split();
                    x.Add(double.Parse(r[0]));
                    y.Add(double.Parse(r[1]));
                }
                
                var sum = 0.0;
                for (int j = 0; j < x.Count - 1; j++)
                {
                    sum += CROSS(x[j], y[j], x[j+1], y[j+1]);
                }
                sum += CROSS(x[x.Count-1], y[x.Count - 1], x[0],y[0]);
                var res = 0.5 * sum;
                Console.WriteLine((res < 0 ? "CW " : "CCW ") + Math.Abs(res).ToString("0.0"));
            }
        }
        
        static double CROSS(double x1, double y1, double x2, double y2) {
            return x1*y2 - x2*y1;
        }
    }
}
