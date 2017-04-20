import java.util.*;

public class main {

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int count = sc.nextInt();
        if(sc.hasNext()){

            String dir = sc.next();

            long nodecount = (long)Math.pow(2,count+1);
            long pos = 1;
            for(char c : dir.toCharArray()){
                pos *=2;
                if(c == 'R')
                    pos++;
            }

            System.out.println(nodecount-pos);
        }
        else
            System.out.println((long)Math.pow(2,count+1)-1);
    }

}
