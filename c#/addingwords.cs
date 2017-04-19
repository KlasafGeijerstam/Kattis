using System;

using System.Collections.Generic;

using System.Linq;



namespace bij

{

    class Program

    {

        static void Main(string[] args)

        {

            Dictionary<string, int> values = new Dictionary<string, int>();

            string[] s;

            string sorg;

            int sum = 0;

            bool invalid = false;

            while ((sorg = Console.ReadLine()) != null)

            {

                s = sorg.Split(' ');

                switch (s[0])

                {

                    case "def":

                        values[s[1]] = int.Parse(s[2]);

                        break;

                    case "calc":

                        for (int i = 1; i < s.Length - 1; i += 2)

                        {

                            if (values.ContainsKey(s[i]))

                            {

                                if (i == 1)

                                    sum += values[s[i]];

                                else if (s[i - 1][0] == '+')

                                    sum += values[s[i]];

                                else if (s[i - 1][0] == '-')

                                    sum -= values[s[i]];

                            }

                            else

                                invalid = true;

                        }

                        Console.Write(sorg.Substring(5) + " ");

                        if (invalid || !values.ContainsValue(sum))

                            Console.WriteLine("unknown");

                        else

                            Console.WriteLine(values.First(x => x.Value == sum).Key);

                        sum = 0;

                        invalid = false;

                        break;

                    case "clear":

                        values.Clear();

                        break;

                }

            }

        }

    }

}