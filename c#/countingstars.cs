using System;

using System.Linq;



public class Program

{

    static char[,] map;

    static int n,m;

    public static void Main()

    {

        string s;

        int caseCount = 1;

        while((s = Console.ReadLine()) != null){

            var ar = s.Split(' ').Select(x => int.Parse(x)).ToArray();  

            n = ar[1];

            m = ar[0];

            

            map = new char[m,n];

            

            for(int i = 0; i < m; i++){

                var str = Console.ReadLine();

                for(int x = 0; x < n; x++)

                    map[i,x] = str[x];

            }

            var starCount = 0;

            for(int y = 0; y < m; y++){

                for(int x = 0; x < n; x++){

                    if(map[y,x] == '-'){

                        flood(x,y);

                        starCount++;

                    }

                }

            }

            Console.WriteLine("Case "+ caseCount++ +": " + starCount);

        }

    }

    static void flood(int x,int y){

        if(x < 0 || x > n-1 || y < 0 || y > m-1)

            return; 

        if(map[y,x] == '#' || map[y,x] == 'X')

            return;

        map[y,x] = 'X';

        flood(x-1,y);

        flood(x+1,y);

        flood(x,y+1);

        flood(x,y-1);

    }

    

}