using System;

                    

public class Program

{

    public static void Main()

    {

        int n = int.Parse(Console.ReadLine());

        if(isPri(n)){

            Console.WriteLine(n-1);

            return;

        }

            for(int i = 2; i <= n; i++){

                if(n % i == 0){

                    Console.WriteLine((n/i)*(i-1));

                    return;

                }

            }

    }

    static bool isPri(long x)

    {

        if(x == 2)

            return true;

        

        if(x % 2 == 0)

            return false;

        for(long i = 3; ; i+= 2)

        {

            long q = x/i;

            if(q <i)

                return true;

            if(x == q*i)

                return false;

        }

}

}