using System;
using System.Linq;
using System.Numerics;
using System.Collections.Generic;
using System.IO;
using System.Diagnostics;
using System.Globalization;

namespace Kattis
{
  public class Program
  {
    public static void Main()
    {
      string str;
      var max = 0;
      var l = new List<int>();
      while((str = Console.ReadLine()) != null)
      {
         l.Add(str.Length);
      }
      max = l.Max();
      l.RemoveAt(l.Count - 1);
      int tot = 0;
      l.ForEach(x => tot += (int)Math.Pow(max - x, 2));
      Console.WriteLine(tot);
    }
  }
}
