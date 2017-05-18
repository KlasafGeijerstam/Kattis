import java.util.*;

public class main {

    public static void main(String[] args) throws Exception{

        Scanner sc = new Scanner(System.in);

        int[] p = new int[4];

        int i1,i2;

        int p1,p2;

        int c1,c2;

        int low = 10000;

        int high = -1;

        p[1] = sc.nextInt();

        p[2] = sc.nextInt();

        p[3] = sc.nextInt();

        i1 = sc.nextInt();

        i2 = sc.nextInt();

        if(i1 < low)

            low = i1;

        if(i2 > high)

            high = i2;

        p1 = sc.nextInt();

        p2 = sc.nextInt();

        if(p1 < low)

            low = p1;

        if(p2 > high)

            high = p2;

        c1 = sc.nextInt();

        c2 = sc.nextInt();

        if(c1 < low)

            low = c1;

        if(c2 > high)

            high = c2;

        int price = 0;

        for (int i = low; i <= high; i++) {

            int tCount = 0;

            if(i >= i1 && i < i2)

                tCount++;

            if(i >= p1 && i < p2)

                tCount++;

            if(i >= c1 && i < c2)

                tCount++;

            price += tCount*p[tCount];

        }

        System.out.println(price);

    }

}