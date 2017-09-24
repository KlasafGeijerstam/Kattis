using System;

namespace Kattis
{
    public class Program
    {
        public static void Main()
        {
            var q = Console.ReadLine();
            var c = int.Parse(Console.ReadLine());
            string s;
            for (int i = 0; i < c; i++)
                if (IsMatch(s = Console.ReadLine(), q))
                    Console.WriteLine(s);                
        }
        
        static bool IsMatch(string input, string pattern)
        {
            var inputPosStack = new int[(input.Length + 1) * (pattern.Length + 1)];  
            var patternPosStack = new int[inputPosStack.Length];                     
            int stackPos = -1, inputPos = 0, patternPos = 0;                                                          
            var pointTested = new bool[input.Length + 1, pattern.Length + 1];       
            var matched = false;

            while (inputPos < input.Length && patternPos < pattern.Length && pattern[patternPos] != '*' && input[inputPos] == pattern[patternPos])
            {
                inputPos++;
                patternPos++;
            }

            if (patternPos == pattern.Length || pattern[patternPos] == '*')
            {
                pointTested[inputPos, patternPos] = true;
                inputPosStack[++stackPos] = inputPos;
                patternPosStack[stackPos] = patternPos;
            }

            while (stackPos >= 0 && !matched)
            {

                inputPos = inputPosStack[stackPos];
                patternPos = patternPosStack[stackPos--];   

                if (inputPos == input.Length && patternPos == pattern.Length)
                    matched = true;
                else
                {  

                    for (int curInputStart = inputPos; curInputStart < input.Length; curInputStart++)
                    {

                        int curInputPos = curInputStart;
                        int curPatternPos = patternPos + 1;

                        if (curPatternPos == pattern.Length)
                            curInputPos = input.Length;
                        else
                        {

                            while (curInputPos < input.Length && curPatternPos < pattern.Length && pattern[curPatternPos] != '*' &&
                                input[curInputPos] == pattern[curPatternPos])
                            {
                                curInputPos++;
                                curPatternPos++;
                            }

                        }

                        if (((curPatternPos == pattern.Length && curInputPos == input.Length) || (curPatternPos < pattern.Length && pattern[curPatternPos] == '*'))
                            && !pointTested[curInputPos, curPatternPos])
                        {
                            pointTested[curInputPos, curPatternPos] = true;
                            inputPosStack[++stackPos] = curInputPos;
                            patternPosStack[stackPos] = curPatternPos;
                        }

                    }
                }

            }
            return matched;
        }
    }
}
