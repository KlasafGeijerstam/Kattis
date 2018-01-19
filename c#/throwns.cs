using System;
using System.Collections.Generic;

public class Program
{
	public static void Main()
	{
		var c = Console.ReadLine().Split();
		int n = int.Parse(c[0]), p = 0;
		var s = new Stack<int>();
		
		var com = Console.ReadLine().Split();
		
		for(int j = 0; j < com.Length; j++) 
		{
			if(com[j] == "undo") {
				for(int i = 0; i < int.Parse(com[j+1]); i++)
					p = s.Pop();
				j++;
			}
			else {
				s.Push(p);
				p = wrap(p + int.Parse(com[j]), n);
			}
		}
		Console.WriteLine(p);
	}
	
	static int wrap(int i, int max) {
		return ((i % max) + max) % max;	
	}
}
