using System;
using System.Linq;
using System.Globalization;
using System.Collections.Generic;
using System.IO;

namespace Kattis
{
    class Program
    {

        enum Day
        {
            Mon = 1,
            Tue = 2,
            Wed = 3,
            Thur = 4,
            Fri = 5,
            Sat = 6,
            Sun = 7
        }

        struct date
        {
            public Day d;
            public int day;
        }

        static void Main(string[] args)
        {
            int cases = int.Parse(Console.ReadLine());
            for(int cas = 0; cas < cases; cas++)
            {
                List<date> l = new List<date>();
                Day curDay = Day.Sun;
                var dm = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToArray();
                int totTdays = dm[0];
                int months = dm[1];

                var mn = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToArray();
                
                foreach(int i in mn)
                {
                    for(int c = 1; c <= i;c++)
                    {
                        l.Add(new date { d = curDay, day = c });    
                        curDay = nextDay(curDay);
                    }
                }
                int dd = 0;
                foreach (date d in l)
                    if (d.d == Day.Fri && d.day == 13)
                        dd++;

                Console.WriteLine(dd);
            }
        }

        static Day nextDay(Day d)
        {
            if ((int)d < 7)
                return ++d;
            else
                return Day.Mon;
        }
    }
}
