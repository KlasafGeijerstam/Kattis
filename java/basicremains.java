import java.math.BigInteger;

import java.util.*;



public class main {



    public static void main(String[] args) {

        Scanner sc = new Scanner(System.in);

        String[] p;

        while((p = sc.nextLine().split(" ")).length != 1){

            int radix = Integer.parseInt(p[0]);

            System.out.println(new BigInteger(p[1],radix).mod(new BigInteger(p[2],radix)).toString(radix));

        }

    }

}