using System;
using System.Linq;
using System.Collections.Generic;       
public class Program
{
    public static void Main()
    {
        var arr = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToArray();
        int h = arr[0];
        int w = arr[1];
        var q = new Queue<int>();
        Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToList().ForEach(x => q.Enqueue(x));
        int curw = 0;
        int curh = 0;
        bool failed = false;
        while(curh < h){
            if(q.Count == 0){
                failed = false;
                break;
            }
            curw += q.Dequeue();
            if(curw == w){
                curw = 0;
                curh++;
            }
            else if(curw > w){
                failed = true;
                break;
            }
        }
        Console.WriteLine(failed ? "NO" : "YES");
    }
}
