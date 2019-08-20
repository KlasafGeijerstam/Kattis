using System;
using System.IO;

namespace ConsoleApplication1
{

    public class Program
    {
        public static void Main()
        {
            long n;
            Scanner sc = new Scanner();
            var buf = new BufferedStdoutWriter();

            while ((n = sc.NextLong()) != 0)
            {
                buf.WriteLine(nextPrime((n * 2) + 1) + (isPri(n) ? "" : " (" + n + " is not prime)"));
            }
            buf.Flush();
        }
        
        static long nextPrime(long x)
    {
        switch(x){
            case 0:
            case 1:
            case 2:
            return 2;
            case 3:
            return 3;
            case 4:
            case 5:
            return 5;
        }
        
        long k = x/6;
        long i = x -6*k;
        long o = i < 2 ? 1: 5;
        x = 6*k+o;
        for(i = (3+o)/2; !isPrime(x); x+= i)
            i ^=6;
        return x;
    }
    
    static bool isPrime(long x)
    {
        if(x % 2 == 0)
            return false;
        long o = 4;
        for(long i = 5;;i +=o)
        {
            long q = x/i;
            if(q < i)
                return true;
            if( x == q*i)
                return false;
            o ^=6;
        }
    }
    static bool isPri(long x)
    {
        if(x == 2)
            return true;
        
        if(x % 2 == 0)
            return false;
        for(long i = 3; ; i+= 2)
        {
            long q = x/i;
            if(q <i)
                return true;
            if(x == q*i)
                return false;
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
    }

}
