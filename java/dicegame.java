import java.util.Scanner;



public class main{

    public static void main(String[] args) throws Exception {

        Scanner sc = new Scanner(System.in);

        float g = 0,e = 0;

        String[] p = sc.nextLine().split(" ");

        int sum = 0;



        for (int i = Integer.parseInt(p[0]); i <= Integer.parseInt(p[1]); i++) {

            sum += i;

        }

        g += sum/(float)(Integer.parseInt(p[1])-Integer.parseInt(p[0])+1);

        sum = 0;

        for (int i = Integer.parseInt(p[2]); i <= Integer.parseInt(p[3]); i++) {

            sum += i;

        }

        g += sum/(float)(Integer.parseInt(p[3])-Integer.parseInt(p[2])+1);



        p = sc.nextLine().split(" ");

        sum = 0;



        for (int i = Integer.parseInt(p[0]); i <= Integer.parseInt(p[1]); i++) {

            sum += i;

        }

        e += sum/(float)(Integer.parseInt(p[1])-Integer.parseInt(p[0])+1);

        sum = 0;

        for (int i = Integer.parseInt(p[2]); i <= Integer.parseInt(p[3]); i++) {

            sum += i;

        }

        e += sum/(float)(Integer.parseInt(p[3])-Integer.parseInt(p[2])+1);



        if(g > e)

            System.out.println("Gunnar");

        else if(e > g)

            System.out.println("Emma");

        else

            System.out.println("Tie");

    }

}