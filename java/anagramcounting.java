

import java.math.BigInteger;

import java.util.HashMap;

import java.util.Scanner;



public class Main{

    public static void main(String[] args) throws Exception{



        Scanner sc = new Scanner(System.in);



        while(sc.hasNextLine()){



            String line = sc.nextLine();

            HashMap<Character,Integer> map = new HashMap<>();

            for(char c: line.toCharArray())

                if(map.containsKey(c))

                    map.replace(c,map.get(c)+1);

                else

                    map.put(c,1);



            BigInteger res = fact(BigInteger.valueOf(line.length()));



            BigInteger div = BigInteger.valueOf(1);

            for(int i : map.values()){

                div = div.multiply(fact(BigInteger.valueOf(i)));

            }





            System.out.println(res.divide(div).toString());



        }

        sc.close();

    }



    public static BigInteger fact(BigInteger i){

        if(i.equals(BigInteger.ONE))

            return i;

        return i.multiply(fact(i.subtract(BigInteger.ONE)));

    }

}