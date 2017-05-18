import java.io.File;

import java.io.FileReader;

import java.io.PrintStream;

import java.util.ArrayList;

import java.util.Scanner;



public class main{



    public static void main(String[] args) throws Exception {

        Scanner sc = new Scanner(System.in);

        boolean frst = true;

        while(sc.hasNextLine()){

            if(!frst)

                System.out.println();

            frst = false;

            String s;

            ArrayList<String> ar = new ArrayList<>();

            int longest = -1;

            while(sc.hasNextLine() && (s = sc.nextLine()).length() > 0){

                ar.add(s);

                if(s.length() > longest)

                    longest = s.length();

            }

            ar.sort((x,y) ->{

               int i = x.length()-1;

               int p = y.length()-1;

               while(i >= 0 && p >= 0){

                   int cmp = Character.compare(x.charAt(i),y.charAt(p));

                   if(cmp != 0)

                       return cmp;

                   i--;

                   p--;

               }

               if(x.length() < y.length())

                   return -1;

               else if(x.length() > y.length())

                   return 1;

               return 0;

            });

            for(String w : ar){

                System.out.printf("%" + longest +"s\n",w);

            }

        }

    }

}