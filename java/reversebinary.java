import java.math.BigInteger;
import java.util.Scanner;
public class Main {
    public static void main(String[] args) throws Exception{
        System.out.println(new BigInteger(new StringBuilder(new BigInteger(new Scanner(System.in).nextLine()).toString(2)).reverse().toString(),2).toString());
    }
}
