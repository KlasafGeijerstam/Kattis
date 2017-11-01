using System;

class Program {
    public static void Main(string[] args) {
        var ca = Console.ReadLine().ToCharArray();
        int s1 = 0, s2 = 0;
        for(int i = 0; i < ca.Length; i++)
            if(i < ca.Length/2)
                s1 += ca[i] - 'A';
            else
                s2 += ca[i] - 'A';
        for(int i = 0; i < ca.Length; i++)
            if(i < ca.Length/2)
                ca[i] = (char)('A' + ((ca[i] - 'A' + s1) % 26));
            else
                ca[i] = (char)('A' + ((ca[i] - 'A' + s2) % 26));
        for(int i = 0; i < ca.Length/2; i++)
            Console.Write((char)('A' + ((ca[i] - 'A' + ca[ca.Length/2 + i]-'A') % 26)));
        Console.WriteLine();
    }
}
