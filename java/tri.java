import java.util.ArrayList;
import java.util.Scanner;

/**
 * Created by dv16kar on 2017-04-10.
 */
public class main {

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int a=  sc.nextInt();
        int b = sc.nextInt();
        int c = sc.nextInt();

        if(a + b == c)
            System.out.println(a + "+" + b + "=" + c);
        else if (a - b == c)
            System.out.println(a + "-" + b + "=" + c);
        else if (a * b == c)
            System.out.println(a + "*" + b + "=" + c);
        else if (b != 0 && a / b == c)
            System.out.println(a + "/" + b + "=" + c);
        else if (a == b + c)
            System.out.println(a + "=" + b + "+" + c);
        else if (a == b - c)
            System.out.println(a + "=" + b + "-" + c);
        else if (a == b * c)
            System.out.println(a + "=" + b + "*" + c);
        else if (c != 0 && a == b / c)
            System.out.println(a + "=" + b + "/" + c);
    }
}
