

import java.util.HashSet;

import java.util.Scanner;

import java.util.Set;



public class Main{

    public static void main(String[] args) throws Exception{



        Scanner sc = new Scanner(System.in);



        while(sc.hasNextInt()){

            int c = sc.nextInt();

            int p = totFactor(c);

            System.out.println(p == 0 ? c + " perfect" : p < 3 ? c + " almost perfect" : c + " not perfect");

        }

        sc.close();

    }



    private static int totFactor(int i){

        int loc = 0;

        int step = i % 2 != 0 ? 2 : 1;

        HashSet<Integer> set = new HashSet<>();

        if(i > 3 && i % 2 != 0)

            return 40;

        set.add(1);

        for(int p = 2; p <= (int)Math.ceil(Math.sqrt(i)); p+= step){

            if(i % p == 0){

                set.add(p);

                set.add(i/p);

            }

        }



        for(int p : set){

            loc += p;

        }

        return Math.abs(i-loc);

    }

}