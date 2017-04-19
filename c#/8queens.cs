using System;

using System.Collections.Generic;

using System.IO;

using System.Linq;



namespace Kattis

{

    class Program

    {

        static char[,] ar = new char[8, 8];

        public static void Main(string[] args)

        {

            for (int i = 0; i < 8; i++)

            {

                var str = Console.ReadLine();

                for (int j = 0; j < 8; j++)

                {

                    ar[j, i] = str[j];

                }

            }

            bool works = true;

            int queens = 0;

            for (int y = 0; y < 8; y++)

            {

                for (int x = 0; x < 8; x++)

                {

                    if(ar[x,y] == '*')

                    {

                        works = check(x, y);

                        queens++;

                        if (!works)

                            goto leb;

                    }

                }

            }

            leb:

            Console.WriteLine(works && queens == 8 ? "valid" : "invalid");

        }

        static bool check(int x,int y)

        {

            //horiz

            //Vert

            for (int i = 0; i < 8; i++)

            {

                if (ar[x, i] == '*' && i != y)

                    return false;

                if (ar[i, y] == '*' && i != x)

                    return false;

            }

            //diag upleft

            int tx = x;

            int ty = y;

            while(tx > 0 && ty > 0)

            {

                tx--;

                ty--;

                if (ar[tx, ty] == '*')

                    return false;

            }

            //diag downright

            tx = x;

            ty = y;

            while (tx < 7 && ty < 7)

            {

                tx++;

                ty++;

                if (ar[tx, ty] == '*')

                    return false;

            }





            tx = x;

            ty = y;

            //upleft

            while (tx < 7 && ty > 0)

            {

                tx++;

                ty--;

                if (ar[tx, ty] == '*')

                    return false;

            }

            //downleft

            tx = x;

            ty = y;

            while (tx > 0 && ty < 7)

            {

                tx--;

                ty++;

                if (ar[tx, ty] == '*')

                    return false;

            }

            return true;

        }



    }

}