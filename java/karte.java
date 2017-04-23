import java.io.BufferedReader;

import java.io.File;

import java.io.FileReader;

import java.io.FileWriter;

import java.util.*;



public class main {



    public static void main(String[] args) throws Exception{

        Scanner sc = new Scanner(System.in);

        HashSet<Card> set = new HashSet<>();

        String s = sc.nextLine();

        int p = 0, k = 0, h = 0, t = 0;

        for (int i = 0; i < s.length(); i+=3) {

            Card card = new Card(s.charAt(i),Integer.parseInt(s.substring(i+1,i+3)));

            if(set.contains(card)) {

                System.out.println("GRESKA");

                return;

            }

            set.add(card);

            if(card.c == 'P')

                p++;

            if(card.c == 'K')

                k++;

            if(card.c == 'H')

                h++;

            if(card.c == 'T')

                t++;

        }

        System.out.println((13-p) + " " + (13-k) + " " + (13-h) + " " +(13-t));

    }



}



class Card{

    char c;

    int v;



    @Override

    public boolean equals(Object o) {

        if (this == o) return true;

        if (o == null || getClass() != o.getClass()) return false;



        Card card = (Card) o;



        if (c != card.c) return false;

        return v == card.v;

    }



    @Override

    public int hashCode() {

        int result = (int) c;

        result = 31 * result + v;

        return result;

    }



    public Card(char c, int v) {

        this.c = c;

        this.v = v;

    }

}