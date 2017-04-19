using System;

using System.Linq;

using System.Collections.Generic;

                    

public class Program

{

    public static void Main()

    {

        Dictionary<int,int> dic = new Dictionary<int,int>();

        Dictionary<int,int> diy = new Dictionary<int,int>();

        for(int i = 0; i < 3; i++)

        {

            var j = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToArray();

            if(dic.ContainsKey(j[0]))

                dic[j[0]]++;

            else

                dic.Add(j[0],1);

            if(diy.ContainsKey(j[1]))

                diy[j[1]]++;

            else

                diy.Add(j[1],1);

        }

        Console.WriteLine("{0} {1}", dic.First(x => x.Value == 1).Key,diy.First(x => x.Value == 1).Key);

    }

}