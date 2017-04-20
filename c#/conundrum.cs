using System;
                    
public class Program
{
    public static void Main()
    {
        var str = Console.ReadLine();
        var count = 0;
            
        for(int i = 0; i < str.Length; i+=3){
            if(str[i] != 'P')
                count++;
            if(str[i+1] != 'E')
                count++;
            if(str[i+2] != 'R')
                count++;
        }
        Console.WriteLine(count);
        
    }
}
