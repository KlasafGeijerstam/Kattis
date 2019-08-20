using System;

namespace Kattis
{
    public class Program
    {
        public static void Main()
        {
            var a = Console.ReadLine().ToCharArray();
            var b = Console.ReadLine().ToCharArray();
            for (int ai = a.Length-1,bi = b.Length-1; ai >= 0 && bi >= 0; ai--,bi--)
            {
                if (a[ai] < b[bi])
                    a[ai] = '#';
                else if(a[ai] > b[bi])
                    b[bi] = '#';
            }
            var ast = new string(a).Replace("#","");
            var bst = new string(b).Replace("#","");
            Console.WriteLine(ast.Length > 0 ? int.Parse(ast).ToString() : "YODA");
            Console.WriteLine(bst.Length > 0 ? int.Parse(bst).ToString() : "YODA");
        }
    }
}
