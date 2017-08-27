using System;
                    
public class Program
{
    static int phi(int n)
    {
        double result = n;

        for (int p=2; p*p<=n; ++p)
        {
            if (n % p == 0)
            {
                while (n % p == 0)
                    n /= p;
                result *= (1.0 - (1.0 / p));
            }
        }
    
        if (n > 1)
            result *= (1.0 - (1.0 / n));
 
        return (int)result;
    }
    
    public static void Main()
    {
        string s;
        while((s = Console.ReadLine()) != "0")
            Console.WriteLine(phi(int.Parse(s)));
    }
}
