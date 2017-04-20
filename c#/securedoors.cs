using System;
using System.Linq;
using System.Collections.Generic;       
public class Program
{
    public static void Main()
    {
        int c = int.Parse(Console.ReadLine());
        var dic = new Dictionary<string,Person>();
        for(int i = 0; i < c; i++){
            var nm = Console.ReadLine().Split(' ');
            if(!dic.ContainsKey(nm[1]))
                dic.Add(nm[1],new Person(nm[1]));
            
            if(nm[0] == "entry"){
                var p = dic[nm[1]];
                Console.WriteLine(p.name + " entered" + ((p.inside) ? " (ANOMALY)" : ""));
                p.inside = true;
            }
            else {
                var p = dic[nm[1]];
                Console.WriteLine(p.name + " exited" + ((!p.inside) ? " (ANOMALY)" : ""));
                p.inside = false;
            }
            
        }
    }
    class Person{
        public string name;
        public bool inside = false;
        public Person(string n){
            name = n;
        }
    }
}
