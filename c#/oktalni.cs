using System;
using System.Numerics;
using System.Text;
                    
public class Program
{
    public static void Main()
    {
        var b = new BigInteger(0);
        
        
        
        var str = Console.ReadLine();
        
        foreach(var c in str){
            b <<= 1;
            b += c == '1' ? 1 : 0;
        }
        
        Console.WriteLine(b.ToOctalString());
        
        
    }
    
    
    
}

public static class ext{
    
    public static string ToOctalString(this BigInteger bigint)
    {
    var bytes = bigint.ToByteArray();
    var idx = bytes.Length - 1;

    // Create a StringBuilder having appropriate capacity.
    var base8 = new StringBuilder(((bytes.Length / 3) + 1) * 8);

    // Calculate how many bytes are extra when byte array is split
    // into three-byte (24-bit) chunks.
    var extra = bytes.Length % 3;

    // If no bytes are extra, use three bytes for first chunk.
    if (extra == 0)
    {
      extra = 3;
    }

    // Convert first chunk (24-bits) to integer value.
    int int24 = 0;
    for (; extra != 0; extra--)
    {
      int24 <<= 8;
      int24 += bytes[idx--];
    }

    // Convert 24-bit integer to octal without adding leading zeros.
    var octal = Convert.ToString(int24, 8);

    // Ensure leading zero exists if value is positive.
    if (octal[0] != '0' && bigint.Sign == 1)
    {
      base8.Append('0');
    }

    // Append first converted chunk to StringBuilder.
    base8.Append(octal);

    // Convert remaining 24-bit chunks, adding leading zeros.
    for (; idx >= 0; idx -= 3)
    {
      int24 = (bytes[idx] << 16) + (bytes[idx - 1] << 8) + bytes[idx - 2];
      base8.Append(Convert.ToString(int24, 8).PadLeft(8, '0'));
    }

    return base8.ToString().Substring(1);
  }
    
}
