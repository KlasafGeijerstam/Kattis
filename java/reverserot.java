import java.util.Collections;
import java.util.Scanner;

public class main{

    public static void main(String[] args) throws Exception {
        Scanner sc = new Scanner(System.in);
        while(true){

            String[] p = sc.nextLine().split(" ");
            int rot = Integer.parseInt(p[0]);
            if(rot == 0)
                break;

            char[] s = p[1].toCharArray();

            for(int k = s.length-1; k >= 0; k--){
                for (int i = 0; i < rot; i++) {
                    s[k] = p(s[k]);
                }
                System.out.print(s[k]);
            }
            System.out.println();
        }
    }

    static char p(char kek){

       if(kek >= 'A' && kek <= 'Y')
            return (char)(kek+1);

        switch(kek){
            case 'Z':
                return '_';
            case '_':
                return '.';
            case '.':
                return 'A';
        }
        return 0;
    }
}
