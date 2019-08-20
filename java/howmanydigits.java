import java.math.BigInteger;
import java.util.*;

public class main {

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        HashMap<Integer,Double> set = new HashMap<>();
        double prev = Math.log10(1);
        set.put(1,1.0);
        set.put(0,1.0);

        for (int i = 2; i < 1000001; i++) {
            set.put(i,prev + Math.log10(i));
            prev += Math.log10(i);
        }

        while(sc.hasNextInt()) {
            System.out.println((int)Math.ceil(set.get(sc.nextInt())));
        }
    }
}
