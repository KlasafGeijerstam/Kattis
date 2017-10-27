using System;
using System.Collections.Generic;
using System.Text.RegularExpressions;

namespace Kattis
{
    class Program
    {
        static void Main(string[] args)
        {
            
            while(1) {
                var str = Console.ReadLine();
                
                if(str == null)
                    return;
                    
                var cases = int.Parse(str);
		        var s = "";
		        var mep = new Dictionary<string, int>();
		        var li = new List<string>();
		        
		        while((s = Console.ReadLine()) != "EndOfText") {
			        var denk = Regex.Split(s.ToLower(), "[^a-z]");
			
			        foreach(var sub in denk) {
				        if(sub.Length < 2)
					        continue;
				
				        if(!mep.ContainsKey(sub))
				            mep[sub] = 0;
				        mep[sub]++;
			        }
		        }
		
		        foreach(var k in mep)
			        if(k.Value == cases)
				        li.Add(k.Key);
		
		        if(li.Count > 0)
			        li.Sort();
			        li.ForEach(x => Console.WriteLine(x));
		        else
			        Console.WriteLine("There is no such word.");
			}
        }
    }
}
