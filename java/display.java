import java.io.File;

import java.io.FileReader;

import java.io.PrintStream;

import java.util.Scanner;



public class main{



    public static void main(String[] args) throws Exception {

        Scanner sc = new Scanner(System.in);

        String s;

        while(!(s = sc.nextLine()).equals("end")){

            String[] ar = new String[7];

            for (int i = 0; i < 7; i++)

                ar[i] = "";

            for (int i = 0; i < s.length(); i++)

                addChar(ar,s.charAt(i),i);

            for (int i = 0; i < 7; i++)

                System.out.println(ar[i]);

            System.out.println("\n");

        }

        System.out.println("end");

    }



    static void addChar(String[] a,char c,int s){

        if(s != 0){

            for (int i = 0; i < 7; i++) {

                a[i] += "  ";

            }

        }

        switch(c){

            case '0':

                a[0] += "+---+";

                a[1] += "|   |";

                a[2] += "|   |";

                a[3] += "+   +";

                a[4] += "|   |";

                a[5] += "|   |";

                a[6] += "+---+";

                break;

            case '1':

                a[0] += "    +";

                a[1] += "    |";

                a[2] += "    |";

                a[3] += "    +";

                a[4] += "    |";

                a[5] += "    |";

                a[6] += "    +";

                break;

            case '2':

                a[0] += "+---+";

                a[1] += "    |";

                a[2] += "    |";

                a[3] += "+---+";

                a[4] += "|    ";

                a[5] += "|    ";

                a[6] += "+---+";

                break;

            case '3':

                a[0] += "+---+";

                a[1] += "    |";

                a[2] += "    |";

                a[3] += "+---+";

                a[4] += "    |";

                a[5] += "    |";

                a[6] += "+---+";

                break;

            case '4':

                a[0] += "+   +";

                a[1] += "|   |";

                a[2] += "|   |";

                a[3] += "+---+";

                a[4] += "    |";

                a[5] += "    |";

                a[6] += "    +";

                break;

            case '5':

                a[0] += "+---+";

                a[1] += "|    ";

                a[2] += "|    ";

                a[3] += "+---+";

                a[4] += "    |";

                a[5] += "    |";

                a[6] += "+---+";

                break;

            case '6':

                a[0] += "+---+";

                a[1] += "|    ";

                a[2] += "|    ";

                a[3] += "+---+";

                a[4] += "|   |";

                a[5] += "|   |";

                a[6] += "+---+";

                break;

            case '7':

                a[0] += "+---+";

                a[1] += "    |";

                a[2] += "    |";

                a[3] += "    +";

                a[4] += "    |";

                a[5] += "    |";

                a[6] += "    +";

                break;

            case '8':

                a[0] += "+---+";

                a[1] += "|   |";

                a[2] += "|   |";

                a[3] += "+---+";

                a[4] += "|   |";

                a[5] += "|   |";

                a[6] += "+---+";

                break;

            case '9':

                a[0] += "+---+";

                a[1] += "|   |";

                a[2] += "|   |";

                a[3] += "+---+";

                a[4] += "    |";

                a[5] += "    |";

                a[6] += "+---+";

                break;

            case ':':

                a[0] += " ";

                a[1] += " ";

                a[2] += "o";

                a[3] += " ";

                a[4] += "o";

                a[5] += " ";

                a[6] += " ";

                break;

        }

    }

}