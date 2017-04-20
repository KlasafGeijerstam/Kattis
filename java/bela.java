import java.util.Scanner;

public class main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int count = sc.nextInt();
        char dom = sc.next().charAt(0);
        int score = 0;
        sc.nextLine();
        for (int i = 0; i < count*4; i++) {
            String s = sc.nextLine();
            score += pv(s.charAt(0),s.charAt(1) == dom);
        }
        System.out.println(score);
        sc.close();
    }

    static int pv(char p, boolean s){

        switch(p){
            case 'A':
                return 11;
            case 'K':
                return 4;
            case 'Q':
                return 3;
            case 'J':
                return s ? 20 : 2;
            case 'T':
                return 10;
            case '9':
                return s ? 14: 0;
            case '8':
                return 0;
            case '7':
                return 0;
        }
        return 1337;
    }
}
