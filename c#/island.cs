using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class Program 
{
    static List<string> map;
    static bool[,] vis;
    static void Main(string[] args) {
        var k = 1;
        while(true) {
            var s = "";
            map = new List<string>();
            do {
                s = Console.ReadLine();
                if(s == null || s.Length == 0) {
                   s = s ?? "EXIT";
                   break;
                } else
                    map.Add(s);
                   
            } while(s.Length > 0);
            Console.WriteLine("Map " + k++.ToString());
            Console.WriteLine("islands: " + islands().ToString());
            Console.WriteLine("bridges: " + bridges());
            Console.WriteLine("buses needed: " + connected().ToString());
            if(s == "EXIT")
                break;
            Console.WriteLine();
        }
    }
    
    static int connected() {
        vis = new bool[map.Count,map[0].Length];
        var c = 0;
        for(int i = 0; i < map.Count; i++) {
            for(int j = 0; j < map[0].Length; j++) {
                if(!vis[i,j] && (map[i][j] == 'X' || map[i][j] == '#')) {
                    c++;  
                    floodC(i,j);
                }
            }
        }
        return c;
    }
    
    static int bridges() {
        vis = new bool[map.Count,map[0].Length];
        var c = 0;
        for(int i = 0; i < map.Count; i++) {
            for(int j = 0; j < map[0].Length; j++) {
                if(!vis[i,j] && map[i][j] == 'B') {
                    c++;  
                    floodB(i,j);
                }
            }
        }
        return c;
    }
    
    static void floodB(int i, int j) {
        vis[i,j] = true;
        if(i + 1 < map.Count && !vis[i+1,j] && map[i+1][j] == 'B')
            floodB(i+1,j);
        if(i - 1 > -1 && !vis[i-1,j] && map[i-1][j] == 'B')
            floodB(i-1,j);
        if(j + 1 < map[0].Length && !vis[i,j+1] && map[i][j+1] == 'B')
            floodB(i,j+1);
        if(j - 1 > -1 && !vis[i,j-1] && map[i][j-1] == 'B')
            floodB(i,j-1);
    }
    
    static void floodC(int i, int j) {
        vis[i,j] = true;
        if(i + 1 < map.Count && !vis[i+1,j] && (map[i+1][j] == 'X' || (map[i+1][j] == '#' && map[i][j] != 'B') || (map[i+1][j] == 'B' && map[i][j] != '#')))
            floodC(i+1,j);
        if(i - 1 > -1 && !vis[i-1,j] && (map[i-1][j] == 'X' || (map[i-1][j] == '#' && map[i][j] != 'B') || (map[i-1][j] == 'B' && map[i][j] != '#')))
            floodC(i-1,j);
        if(j + 1 < map[0].Length && !vis[i,j+1] && (map[i][j+1] == 'X' || (map[i][j+1] == '#' && map[i][j] != 'B') || (map[i][j+1] == 'B' && map[i][j] != '#')))
            floodC(i,j+1);
        if(j - 1 > -1 && !vis[i,j-1] && (map[i][j-1] == 'X' || (map[i][j-1] == '#' && map[i][j] != 'B') || (map[i][j-1] == 'B' && map[i][j] != '#')))
            floodC(i,j-1);
    }
    
    static int islands() {
        vis = new bool[map.Count,map[0].Length];
        var c = 0;
        for(int i = 0; i < map.Count; i++) {
            for(int j = 0; j < map[0].Length; j++) {
                if(!vis[i,j] && (map[i][j] == 'X' || map[i][j] == '#')) {
                    c++;  
                    flood(i,j);
                }
            }
        }
        return c;
    }
    
    static void flood(int i, int j) {
        vis[i,j] = true;
        if(i + 1 < map.Count && !vis[i+1,j] && (map[i+1][j] == 'X' || map[i+1][j] == '#'))
            flood(i+1,j);
        if(i - 1 > -1 && !vis[i-1,j] && (map[i-1][j] == 'X' || map[i-1][j] == '#'))
            flood(i-1,j);
        if(j + 1 < map[0].Length && !vis[i,j+1] && (map[i][j+1] == 'X' || map[i][j+1] == '#'))
            flood(i,j+1);
        if(j - 1 > -1 && !vis[i,j-1] && (map[i][j-1] == 'X' || map[i][j-1] == '#'))
            flood(i,j-1);
    }
}
