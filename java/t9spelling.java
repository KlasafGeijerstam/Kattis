import java.util.Scanner;

public class Main{

    public static void main(String[] args){
        Scanner sc = new Scanner(System.in);
        int len = Integer.parseInt(sc.nextLine());
        for(int i = 0; i < len; i++){
            String s = sc.nextLine();
            String out = "Case #" + (i+1) + ": ";
            for(char c : s.toCharArray()){
                String tmp = "";
                switch(c){
                    case 'a':
                        tmp = "2";
                        break;
                    case 'b':
                        tmp = "22";
                        break;
                    case 'c':
                        tmp = "222";
                        break;
                    case 'd':
                        tmp = "3";
                        break;
                    case 'e':
                        tmp = "33";
                        break;
                    case 'f':
                        tmp = "333";
                        break;
                    case 'g':
                        tmp = "4";
                        break;
                    case 'h':
                        tmp = "44";
                        break;
                    case 'i':
                        tmp = "444";
                        break;
                    case 'j':
                        tmp = "5";
                        break;
                    case 'k':
                        tmp = "55";
                        break;
                    case 'l':
                        tmp = "555";
                        break;
                    case 'm':
                        tmp = "6";
                        break;
                    case 'n':
                        tmp = "66";
                        break;
                    case 'o':
                        tmp = "666";
                        break;
                    case 'p':
                        tmp = "7";
                        break;
                    case 'q':
                        tmp = "77";
                        break;
                    case 'r':
                        tmp = "777";
                        break;
                    case 's':
                        tmp = "7777";
                        break;
                    case 't':
                        tmp = "8";
                        break;
                    case 'u':
                        tmp = "88";
                        break;
                    case 'v':
                        tmp = "888";
                        break;
                    case 'w':
                        tmp = "9";
                        break;
                    case 'x':
                        tmp = "99";
                        break;
                    case 'y':
                        tmp = "999";
                        break;
                    case 'z':
                        tmp = "9999";
                        break;
                    case ' ':
                        tmp = "0";
                        break;
                }
                if(out.charAt(out.length()-1) == tmp.charAt(0))
                    out += " ";
                out += tmp;
            }
            System.out.println(out);
        }

        sc.close();
    }




}
