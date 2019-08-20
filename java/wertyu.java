import java.util.Scanner;

public class main{

    public static void main(String[] args) throws Exception {
        Scanner sc = new Scanner(System.in);

        while(sc.hasNextLine()){
            char[] s = sc.nextLine().toCharArray();
            for(char c : s)
                System.out.print(p(c));
            System.out.println();
        }
    }

    static char p(char kek){
        switch (kek){
            case 'A':
                kek = 'A';
                break;
            case 'B':
                kek = 'V';
                break;
            case 'C':
                kek = 'X';
                break;
            case 'D':
                kek = 'S';
                break;
            case 'E':
                kek = 'W';
                break;
            case 'F':
                kek = 'D';
                break;
            case 'G':
                kek = 'F';
                break;
            case 'H':
                kek = 'G';
                break;
            case 'I':
                kek = 'U';
                break;
            case 'J':
                kek = 'H';
                break;
            case 'K':
                kek = 'J';
                break;
            case 'L':
                kek = 'K';
                break;
            case 'M':
                kek = 'N';
                break;
            case 'N':
                kek = 'B';
                break;
            case 'O':
                kek = 'I';
                break;
            case 'P':
                kek = 'O';
                break;
            case 'Q':
                kek = 'Q';
                break;
            case 'R':
                kek = 'E';
                break;
            case 'S':
                kek = 'A';
                break;
            case 'T':
                kek = 'R';
                break;
            case 'U':
                kek = 'Y';
                break;
            case 'V':
                kek = 'C';
                break;
            case 'W':
                kek = 'Q';
                break;
            case 'X':
                kek = 'Z';
                break;
            case 'Y':
                kek = 'T';
                break;
            case 'Z':
                kek = 'Z';
                break;
            case ';':
                kek = 'L';
                break;
            case '\'':
                kek = ';';
                break;
            case '[':
                kek = 'P';
                break;
            case ']':
                kek = '[';
                break;
            case '\\':
                kek = ']';
                break;
            case ',':
                kek = 'M';
                break;
            case '.':
                kek = ',';
                break;
            case '/':
                kek = '.';
                break;
            case '=':
                kek = '-';
                break;
            case '-':
                kek = '0';
                break;
            case '0':
                kek = '9';
                break;
            case '9':
                kek = '8';
                break;
            case '8':
                kek = '7';
                break;
            case '7':
                kek = '6';
                break;
            case '6':
                kek = '5';
                break;
            case '5':
                kek = '4';
                break;
            case '4':
                kek = '3';
                break;
            case '3':
                kek = '2';
                break;
            case '2':
                kek = '1';
                break;
            case '1':
                kek = '‘';
                break;
            case '‘':
                kek = '‘';
                break;
            case ' ':
                kek = ' ';
        }
        return kek;
    }
}
