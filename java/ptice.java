import java.math.BigInteger;
import java.util.*;

public class main {

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        sc.nextLine();
        String s = sc.nextLine();

        int adr = 0;
        int gor = 0;
        int bru = 0;
        String adrs = "ABC";
        String brus = "BABC";
        String gors = "CCAABB";
        for (int i = 0; i < s.length(); i++) {
            if(s.charAt(i) == adrs.charAt(i%3))
                adr++;
            if(s.charAt(i) == brus.charAt(i%4))
                bru++;
            if(s.charAt(i) == gors.charAt(i%6))
                gor++;
        }
        int max = Math.max(adr,Math.max(gor,bru));
        System.out.println(max);
        if(adr == max)
            System.out.println("Adrian");
        if(bru == max)
            System.out.println("Bruno");
        if(gor == max)
            System.out.println("Goran");
    }
}
