import java.math.BigInteger;
import java.util.*;

public class main {

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        while(sc.hasNextLine()){
            ArrayList<Integer> arr = new ArrayList<>();
            Arrays.asList(sc.nextLine().split(" ")).forEach(x -> arr.add(Integer.parseInt(x)));
            for (int i = 0; i < arr.size(); i++) {
                int sum = 0;
                for (int j = 0; j < arr.size(); j++) {
                    if(j != i){
                        sum += arr.get(j);
                    }
                }
                if(sum == arr.get(i)){
                    System.out.println(sum);
                    break;
                }
            }
        }
    }
}
