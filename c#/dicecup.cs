using System;
using System.Collections.Generic;
using System.Linq;
using System.IO;
namespace Kattis
{

  public class Program
  {
    
    public static void Main()
    {
        var d = new Dictionary<int,int>();
        var pp = Console.ReadLine().Split(' ').Select(x => int.Parse(x)).ToArray();
        int i1 = pp[0];
        int i2 = pp[1];
        for(int j = 1; j <= i1; j++)
        {
          for(int k = 1; k <= i2; k++)
          {
            if(d.ContainsKey(j+k))
              d[j+k]++;
            else
              d[j+k] = 1;
          }
        
        }
        var nums = new List<int>();
        int max = -1;
        foreach(var kvp in d)
        {
          if(kvp.Value == max)  
            nums.Add(kvp.Key);
           else if(kvp.Value > max)
           {
            nums.Clear();
            nums.Add(kvp.Key);
            max = kvp.Value;
           } 
            
        }
        nums.Sort();
        nums.ForEach(x => Console.WriteLine(x));
        
    }
  }
}
