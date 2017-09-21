using System;

namespace Kattis
{
    public class Program
    {

        static int Get(string[] s)
        {
            if      (s[0] == "xxxxx" && s[1] == "x...x" && s[2] == "x...x" && s[3] == "x...x" && s[4] == "x...x" && s[5] == "x...x" && s[6] == "xxxxx")
                return 0;
            else if (s[0] == "....x" && s[1] == "....x" && s[2] == "....x" && s[3] == "....x" && s[4] == "....x" && s[5] == "....x" && s[6] == "....x")
                return 1;
            else if (s[0] == "xxxxx" && s[1] == "....x" && s[2] == "....x" && s[3] == "xxxxx" && s[4] == "x...." && s[5] == "x...." && s[6] == "xxxxx")
                return 2;
            else if (s[0] == "xxxxx" && s[1] == "....x" && s[2] == "....x" && s[3] == "xxxxx" && s[4] == "....x" && s[5] == "....x" && s[6] == "xxxxx")
                return 3;
            else if (s[0] == "x...x" && s[1] == "x...x" && s[2] == "x...x" && s[3] == "xxxxx" && s[4] == "....x" && s[5] == "....x" && s[6] == "....x")
                return 4;
            else if (s[0] == "xxxxx" && s[1] == "x...." && s[2] == "x...." && s[3] == "xxxxx" && s[4] == "....x" && s[5] == "....x" && s[6] == "xxxxx")
                return 5;
            else if (s[0] == "xxxxx" && s[1] == "x...." && s[2] == "x...." && s[3] == "xxxxx" && s[4] == "x...x" && s[5] == "x...x" && s[6] == "xxxxx")
                return 6;
            else if (s[0] == "xxxxx" && s[1] == "....x" && s[2] == "....x" && s[3] == "....x" && s[4] == "....x" && s[5] == "....x" && s[6] == "....x")
                return 7;
            else if (s[0] == "xxxxx" && s[1] == "x...x" && s[2] == "x...x" && s[3] == "xxxxx" && s[4] == "x...x" && s[5] == "x...x" && s[6] == "xxxxx")
                return 8;
            else if (s[0] == "xxxxx" && s[1] == "x...x" && s[2] == "x...x" && s[3] == "xxxxx" && s[4] == "....x" && s[5] == "....x" && s[6] == "xxxxx")
                return 9;
            else
                return -1;
        }

        static string[] Gets(int i)
        {
            var s = new string[7];
            if (i == 0)
            {
                s[0] = "xxxxx" ; s[1] = "x...x" ; s[2] = "x...x" ; s[3] = "x...x" ; s[4] = "x...x" ; s[5] = "x...x" ; s[6] = "xxxxx";
            }
            else if (i == 1)
            {
                s[0] = "....x" ; s[1] = "....x" ; s[2] = "....x" ; s[3] = "....x" ; s[4] = "....x" ; s[5] = "....x" ; s[6] = "....x";
            }
            else if (i == 2)
            {
                s[0] = "xxxxx" ; s[1] = "....x" ; s[2] = "....x" ; s[3] = "xxxxx" ; s[4] = "x...." ; s[5] = "x...." ; s[6] = "xxxxx";
            }
            else if (i == 3)
            {
                s[0] = "xxxxx" ; s[1] = "....x" ; s[2] = "....x" ; s[3] = "xxxxx" ; s[4] = "....x" ; s[5] = "....x" ; s[6] = "xxxxx";
            }
            else if (i == 4)
            {
                s[0] = "x...x" ; s[1] = "x...x" ; s[2] = "x...x" ; s[3] = "xxxxx" ; s[4] = "....x" ; s[5] = "....x" ; s[6] = "....x";
            }
            else if (i == 5)
            {
                s[0] = "xxxxx" ; s[1] = "x...." ; s[2] = "x...." ; s[3] = "xxxxx" ; s[4] = "....x" ; s[5] = "....x" ; s[6] = "xxxxx";
            }
            else if (i == 6)
            {
                s[0] = "xxxxx" ; s[1] = "x...." ; s[2] = "x...." ; s[3] = "xxxxx" ; s[4] = "x...x" ; s[5] = "x...x" ; s[6] = "xxxxx";
            }
            else if (i == 7)
            {
                s[0] = "xxxxx" ; s[1] = "....x" ; s[2] = "....x" ; s[3] = "....x" ; s[4] = "....x" ; s[5] = "....x" ; s[6] = "....x";
            }
            else if (i == 8)
            {
                s[0] = "xxxxx" ; s[1] = "x...x" ; s[2] = "x...x" ; s[3] = "xxxxx" ; s[4] = "x...x" ; s[5] = "x...x" ; s[6] = "xxxxx";
            }
            else if (i == 9)
            {
                s[0] = "xxxxx" ; s[1] = "x...x" ; s[2] = "x...x" ; s[3] = "xxxxx" ; s[4] = "....x" ; s[5] = "....x" ; s[6] = "xxxxx";
            }
            return s;
        }

        public static void Main()
        {
            var p = new string[7];
            for (int j = 0; j < 7; j++)
                p[j] = Console.ReadLine();

            var str = "";
            var str2 = "";
            for (int i = 0; i < p[0].Length; i += 6)
            {
                var tmp = new string[7];
                for (int j = 0; j < 7; j++)
                    tmp[j] = p[j].Substring(i, 5);
                var res = Get(tmp);
                if(res == -1)
                {
                    str2 = str;
                    str = "";
                    continue;
                }
                str += res;
            }
            var result = long.Parse(str) + long.Parse(str2);

            for (int j = 0; j < 7; j++)
            {
                var r = "";
                var pep = result.ToString();
                for (int i = 0; i < pep.Length; i++)
                    r += Gets(pep[i] - '0')[j] + (i < pep.Length - 1 ? "." : "");
                Console.WriteLine(r);
            }
        }
    }
} 
