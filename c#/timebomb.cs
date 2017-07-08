using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Threading;

namespace Kattis
{
    public class Program
    {
        
        static int Get(string[] s)
        {
            if (s[0] == "***" && s[1] == "* *" && s[2] == "* *" && s[3] == "* *" && s[4] == "***")
                return 0;
            else if (s[0] == "  *" && s[1] == "  *" && s[2] == "  *" && s[3] == "  *" && s[4] == "  *")
                return 1;
            else if (s[0] == "***" && s[1] == "  *" && s[2] == "***" && s[3] == "*  " && s[4] == "***")
                return 2;
            else if (s[0] == "***" && s[1] == "  *" && s[2] == "***" && s[3] == "  *" && s[4] == "***")
                return 3;
            else if (s[0] == "* *" && s[1] == "* *" && s[2] == "***" && s[3] == "  *" && s[4] == "  *")
                return 4;
            else if (s[0] == "***" && s[1] == "*  " && s[2] == "***" && s[3] == "  *" && s[4] == "***")
                return 5;
            else if (s[0] == "***" && s[1] == "*  " && s[2] == "***" && s[3] == "* *" && s[4] == "***")
                return 6;
            else if (s[0] == "***" && s[1] == "  *" && s[2] == "  *" && s[3] == "  *" && s[4] == "  *")
                return 7;
            else if (s[0] == "***" && s[1] == "* *" && s[2] == "***" && s[3] == "* *" && s[4] == "***")
                return 8;
            else if (s[0] == "***" && s[1] == "* *" && s[2] == "***" && s[3] == "  *" && s[4] == "***")
                return 9;
            else
                return -1;
        }

        public static void Main()
        {
            var p = new string[5];
            for (int j = 0; j < 5; j++)
                p[j] = Console.ReadLine();

            var str = "";
            for (int i = 0; i < p[0].Length; i+=4)
            {
                var tmp = new string[5];
                for (int j = 0; j < 5; j++)
                    tmp[j] = p[j].Substring(i, 3);
                var res = Get(tmp);
                if(res == -1)
                {
                    Console.WriteLine("BOOM!!");
                    return;
                }
                str += res;
            }
            if(int.Parse(str)%6 == 0)
                Console.WriteLine("BEER!!");
            else
                Console.WriteLine("BOOM!!");
        }
    }
}
