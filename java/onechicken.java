import java.util.ArrayList;
import java.util.Scanner;

/**
 * Created by dv16kar on 2017-04-10.
 */
public class main {

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int m = sc.nextInt();
        if(n > m){
            System.out.println("Dr. Chaz needs " + (n - m) + " more" + (n-m == 1 ? " piece" : " pieces") + " of chicken!");
        }
        else{
            System.out.println("Dr. Chaz will have " + (m - n) + (m-n == 1 ? " piece" : " pieces") + " of chicken left over!");
        }

    }
}
