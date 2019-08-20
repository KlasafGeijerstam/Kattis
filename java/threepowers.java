import java.math.BigInteger;
import java.util.Scanner;
public class Main {
    public static void main(String[] args) throws Exception{
        Scanner sc = new Scanner(System.in);
        String p;
        BigInteger b = new BigInteger("3");
        while(!(p = sc.nextLine()).equals("0")){
            p = new StringBuilder(new BigInteger(p).subtract(BigInteger.ONE).toString(2)).reverse().toString();
            StringBuilder sb = new StringBuilder();
            if(p.equals("0")){
                System.out.println("{ }");
                continue;
            }
            int index = 0;
            boolean first = true;
            sb.append("{ ");
            for (char c : p.toCharArray()) {
                if(c == '1'){
                    if(!first)
                        sb.append(", ");
                    sb.append(b.pow(index).toString());
                    first = false;
                }
                index++;
            }
            System.out.println(sb.append(" }"));
        }
    }
}
