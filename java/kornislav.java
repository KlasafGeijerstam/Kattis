import java.util.Arrays;

import java.util.HashSet;

import java.util.Scanner;



public class Main{

    public static void main(String[] args) throws Exception{

        Scanner sc = new Scanner(System.in);

        int[] ar = new int[] {sc.nextInt(),sc.nextInt(),sc.nextInt(),sc.nextInt()};

        Arrays.sort(ar);

        System.out.println(ar[2] * ar[0]);

        sc.close();

    }



}