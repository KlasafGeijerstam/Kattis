import java.math.BigInteger;
import java.util.*;

public class main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int count = sc.nextInt();
        ArrayList<Integer> arr = new ArrayList<>();
        for (int i = 0; i < count; i++) {
            arr.add(sc.nextInt());
        }
        Collections.sort(arr);
        Collections.reverse(arr);
        int totPrice = 0;
        for (int i = 0;;i++) {
            if(i + 3 <= arr.size()){
                totPrice += arr.get(i);
                totPrice += arr.get(i+1);
                i+=2;
            }
            else{
                for (int j = i; j < arr.size(); j++) {
                    totPrice += arr.get(j);
                }
                break;
            }
        }
        System.out.println(totPrice);

    }
}
