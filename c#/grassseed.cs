using System;
                    
public class Program
{
    public static void Main()
    {
        double price = double.Parse(Console.ReadLine());
        int amount = int.Parse(Console.ReadLine());
        double tp = 0;
        for(int i = 0; i < amount; i++){
            var p = Console.ReadLine().Split(' ');
            tp += price * double.Parse(p[0]) * double.Parse(p[1]);
        }
        Console.WriteLine(tp);
    }
    
}
