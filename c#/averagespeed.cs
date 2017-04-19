using System;

using System.Linq;

using System.Globalization;

using System.Collections.Generic;

using System.IO;



namespace Kattis

{

    class Program

    {



        struct time

        {

            public int s;

            public int h;

            public int m;



            public static int operator -(time t,time t1)

            {

                return (t.s + 60 * t.m + 3600 * t.h) - (t1.s + 60 * t1.m + 3600 * t1.h);

            }

            public override string ToString()

            {

                return h.ToString("D2") + ":" + m.ToString("D2") + ":" + s.ToString("D2");

            }

        }

        static void Main(string[] args)

        {

            string s;

            double distance = 0;

            double speed = 0;

            //DateTime prevTime = new DateTime(1, 1, 1, 0, 0, 1);

            time prevTime = new Kattis.Program.time() { h = 0, m = 0, s = 0 };

            while ((s = Console.ReadLine()) != null)

            {

                try

                {

                    string[] a;

                    if ((a = s.Split(' ')).Length > 1)

                    {

                        var c = a[0].Split(':');

                        //var time = new DateTime(1, 1, 1, int.Parse(c[0]), int.Parse(c[1]), int.Parse(c[2]));

                        var time = new time() { h = int.Parse(c[0]), m = int.Parse(c[1]), s = int.Parse(c[2]) };

                        distance += (time - prevTime) * (speed);

                        speed = kp(int.Parse(a[1]));

                        prevTime = time;

                    }

                    else

                    {



                        var c = a[0].Split(':');

                        //var time = new DateTime(1, 1, 1, int.Parse(c[0]), int.Parse(c[1]), int.Parse(c[2]));

                        var time = new time() { h = int.Parse(c[0]), m = int.Parse(c[1]), s = int.Parse(c[2]) };

                        Console.WriteLine(time.ToString() + " " + Math.Round(((distance) + (time - prevTime) * speed) / 1000, 2).ToString("F2", CultureInfo.InvariantCulture) + " km");

                    }

                }

                catch (Exception)

                {



                    throw;

                }

            }

        }



        static double kp(double a)

        {

            return a / 3.6;

        }

    }

}