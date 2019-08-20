import java.util.Scanner;

public class main{

    public static void main(String[] args) throws Exception {

        Scanner sc = new Scanner(System.in);
        String big = sc.nextLine();
        if(big.length() < 10)
        {
            long c = 1;
            long num = Long.parseLong(big);
            while (true)
            {
                num /= c++;
                if (num == 1)
                {
                    System.out.println(c - 1);
                    return;
                }

            }
        }

        long i = 0;
        double a = 1;
        int len = big.length();
        while (++i > 0)
        {
            a += Math.log10(i);
            if(Math.floor(a) == len)
            {
                System.out.println(i);
                return;
            }
        }
    }
}
