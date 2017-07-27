using System;
using BasicInterpreter;

namespace Kattis
{
    public class Program
    {
        public static void Main()
        {
            string s;
            while ((s = Console.ReadLine()) != null)
            {
                Interpreter.AddCommand(s);
            }
            Interpreter.Run();
        }
    }
}
