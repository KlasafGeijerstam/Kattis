using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;

public class Program
{
	public static void Main()
	{
		string line;
		while((line = Console.ReadLine()) != null) {
			var nums = new List<BigInteger>(line.Split().Select(x => BigInteger.Parse(x)));
			var result = nums[0];
			for(int i = 1; i < nums.Count; i++)
				result *= nums[i] / BigInteger.GreatestCommonDivisor(nums[i], result);
			Console.WriteLine(result);
		}
	}
}
