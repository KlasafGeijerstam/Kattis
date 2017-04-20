using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace Kattis
{
    class Program
    {

        static void Main(string[] args)
        {
            Scanner sc = new Scanner ();
            int hc = sc.NextInt();
            int nc = sc.NextInt();
            Dictionary<int,Node> hses = new Dictionary<int,Node>();


            for(int i = 0; i < hc; i++){
                hses.Add(i+1, new Node(i+1));
            }
            for(int i = 0; i < nc; i++){
                int h1 = sc.NextInt();
                int h2 = sc.NextInt();


                Node hs1 = hses[h1];
                Node hs2 = hses[h2];

                hs1.cns.Add(hs2);
                hs2.cns.Add(hs1);

            }

            Node internetz = hses[1];
            hasInternetz(internetz);
            bool wr = false;
            foreach(var n in hses){
                if(!n.Value.visited){
                    Console.WriteLine(n.Value.num);
                    wr = true;
                }
            }

            if(!wr)
                Console.WriteLine("Connected");

        }
        private static void hasInternetz(Node n){
            n.visited = true;
            for(int i = 0; i < n.cns.Count; i++){
                if(!n.cns [i].visited){
                    hasInternetz(n.cns[i]);
                }
            }
        }

    }

    class Node {
        public List<Node> cns = new List<Node>();
        public int num;
        public bool visited = false;
        public Node(int num){
            this.num = num;
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
