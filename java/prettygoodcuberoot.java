import java.math.BigInteger;
import java.math.MathContext;
import java.util.Scanner;
import java.math.BigDecimal;

public class Main {
    public static void main(String[] args) throws Exception {
        Scanner sc = new Scanner(System.in);
        while (sc.hasNextLine()) {
            BigInteger n = new BigInteger(sc.nextLine());
            BigInteger ai = new BigInteger(n.toString());
            while (ai.pow(3).compareTo(n) == 1)
                ai = (ai.multiply(BigInteger.valueOf(2)).add(n.divide(ai.pow(2)))).divide(BigInteger.valueOf(3));

            if (n.subtract(ai.pow(3)).abs().compareTo(n.subtract(ai.add(BigInteger.ONE).pow(3)).abs()) == 1)
                ai = ai.add(BigInteger.ONE);
            System.out.println(ai);
        }


    }
}
