import java.util.Scanner;

public class Main{

    public static void main(String[] args) throws Exception{

        Scanner sc = new Scanner(System.in);
        primeFactors(sc.nextInt());

        sc.close();
    }

    private static void primeFactors(int i){
        int count = 0;
        while(i % 2 == 0){
            count++;
            i /=2;
        }

        for(int j = 3; j <= Math.sqrt(i); j+= 2){
            while(i%j == 0){
                count++;
                i /=j;
            }
        }

        if(i > 2)
            count++;

        System.out.println(count);
    }



}
