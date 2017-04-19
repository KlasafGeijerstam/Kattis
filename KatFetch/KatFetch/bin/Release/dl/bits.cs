



import java.util.HashSet;

import java.util.Scanner;



public class Main{



    private static HashSet<Long> set = new HashSet<>();



    public static void main(String[] args) throws Exception{



        Scanner sc = new Scanner(System.in);

        int len = Integer.parseInt(sc.nextLine());

        for(int i = 0 ; i < len; i++) {

            String n = sc.nextLine();

            String curString = "";

            int bits = 0;

            int count = 0;

            for (int j = 0; j < n.length(); j++) {

                curString += String.valueOf(n.charAt(j));

                String num = Integer.toBinaryString(Integer.parseInt(curString));



                for(int c = 0; c < num.length(); c++)

                    if(num.charAt(c) == '1')

                        count++;

                if(count >= bits)

                    bits = count;

                count = 0;

            }

            System.out.println(bits);

        }

        sc.close();



    }



}