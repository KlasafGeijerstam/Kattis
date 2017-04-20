import java.util.ArrayList;
import java.util.Scanner;

/**
 * Created by dv16kar on 2017-04-10.
 */
public class main {

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int l =  sc.nextInt();
        int d = sc.nextInt();
        int x = sc.nextInt();
        int n = -1;
        int m = -1;
        for (int i = l; i <= d; i++) {
            if(sum(i) == x){
                n = i;
                break;
            }
        }
        for (int i = d; i >= 0; i--) {
            if(sum(i) == x){
                m = i;
                break;
            }

        }
        System.out.println(n +"\n"+m);
    }

    static int sum(int p){
        char[] str = Integer.toString(p).toCharArray();
        int sum = 0;
        for(char c : str){
            sum += Integer.parseInt(String.valueOf(c));
        }
        return sum;
    }
}
