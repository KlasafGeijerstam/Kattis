using System;
namespace Kattis
{
    class Katt
    {
        public static void Main(string[] args)
        {
            var p = int.Parse(Console.ReadLine())-1;
            Console.ReadLine();
            var t = 3*60+30;
            while(true)
            {
                var a = Console.ReadLine().Split();
                var ti = int.Parse(a[0]);
                if(t - ti <= 0){
                    Console.WriteLine(p+1);
                    return;
                }
                t -= ti;
                if(a[1] == "T")
                    p = (p+1)%8;
            }
        }
    }
}
