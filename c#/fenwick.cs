using System;
using System.Linq;
using System.Numerics;
using System.IO;

public class Program
{
    public static void Main()
    {

        var writer = new BufferedStdoutWriter();
        var sc = new Scanner();
        var i1 = sc.NextInt();
        var i2 = sc.NextInt();
        var ba = new long[i1];

        writer.AutoFlush = false;

        for(;i2 > 0; i2--){
            var s = sc.Next();
            long ind = sc.NextLong();
            if(s == "+"){
                var b = sc.NextLong();
                while (ind < ba.Length) {
                    ba [ind] += b;
                    ind += ((ind+1) & -(ind+1));
                }
            }
            else{
                
                var sum = 0L;
                while (ind != 0) {
                    sum += ba [ind - 1];
                    ind -= ((ind) & -(ind));
                }
                writer.WriteLine (sum);
            }
        }
        writer.Flush();
    }
    static long LSB(long a){
        return a & -a;
    }
         


}

public class NoMoreTokensException : Exception
{
}

public class Tokenizer
{
    string[] tokens = new string[0];
    private int pos;
    StreamReader reader;

    public Tokenizer(Stream inStream)
    {
        var bs = new BufferedStream(inStream);
        reader = new StreamReader(bs);
    }

    public Tokenizer() : this(Console.OpenStandardInput())
    {
        // Nothing more to do
    }

    private string PeekNext()
    {
        if (pos < 0)
            // pos < 0 indicates that there are no more tokens
            return null;
        if (pos < tokens.Length)
        {
            if (tokens[pos].Length == 0)
            {
                ++pos;
                return PeekNext();
            }
            return tokens[pos];
        }
        string line = reader.ReadLine();
        if (line == null)
        {
            // There is no more data to read
            pos = -1;
            return null;
        }
        // Split the line that was read on white space characters
        tokens = line.Split(null);
        pos = 0;
        return PeekNext();
    }

    public bool HasNext()
    {
        return (PeekNext() != null);
    }

    public string Next()
    {
        string next = PeekNext();
        if (next == null)
            throw new NoMoreTokensException();
        ++pos;
        return next;
    }
}


public class Scanner : Tokenizer
{

    public int NextInt()
    {
        return int.Parse(Next());
    }

    public long NextLong()
    {
        return long.Parse(Next());
    }

    public float NextFloat()
    {
        return float.Parse(Next());
    }

    public double NextDouble()
    {
        return double.Parse(Next());
    }
}


public class BufferedStdoutWriter : StreamWriter
{
    public BufferedStdoutWriter() : base(new BufferedStream(Console.OpenStandardOutput()))
    {
    }
}
