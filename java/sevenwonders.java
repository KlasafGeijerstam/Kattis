import java.io.BufferedReader;

import java.io.File;

import java.io.FileReader;

import java.io.FileWriter;

import java.util.*;



public class main {



    public static void main(String[] args) throws Exception{

        Scanner sc = new Scanner(System.in);

        char[] str = sc.nextLine().toCharArray();

        int t = 0;

        int g = 0;

        int c = 0;

        ArrayList<Character> arg = new ArrayList<>();

        for(char k : str){

            switch(k){

                case 'T':

                    t++;

                    break;

                case 'C':

                    c++;

                    break;

                case 'G':

                    g++;

                    break;

            }

            arg.add(k);

        }

        int score = (int)Math.pow(t,2) + (int)Math.pow(c,2) + (int)Math.pow(g,2);



        while(arg.contains('T') && arg.contains('G') && arg.contains('C')){

            score += 7;

            arg.remove((Object)'T');

            arg.remove((Object)'G');

            arg.remove((Object)'C');

        }





        System.out.println(score);

    }



}