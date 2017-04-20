import java.math.BigInteger;
import java.util.*;

public class main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        while(sc.hasNextLine()){
            char[] s = sc.nextLine().toCharArray();
            ArrayList<Integer> lens = new ArrayList<>();
            String message = "";
            for(char c : s){
                String p = toMorse(String.valueOf(c));
                lens.add(lens.size(),p.length());
                message += p;
            }

            Collections.reverse(lens);
            int index = 0;
            for(int len : lens){
                System.out.print(fromMorse(message.substring(index,index+len)));
                index += len;
            }
            System.out.println();

        }
    }

    static String toMorse(String s){
        switch(s){
            case "A":
                return ".-";
            case "B":
                return "-...";
            case "C":
                return "-.-.";
            case "D":
                return "-..";
            case "E":
                return ".";
            case "F":
                return "..-.";
            case "G":
                return "--.";
            case "H":
                return "....";
            case "I":
                return "..";
            case "J":
                return ".---";
            case "K":
                return "-.-";
            case "L":
                return ".-..";
            case "M":
                return "--";
            case "N":
                return "-.";
            case "O":
                return "---";
            case "P":
                return ".--.";
            case "Q":
                return "--.-";
            case "R":
                return ".-.";
            case "S":
                return "...";
            case "T":
                return "-";
            case "U":
                return "..-";
            case "V":
                return "...-";
            case "W":
                return ".--";
            case "X":
                return "-..-";
            case "Y":
                return "-.--";
            case "Z":
                return "--..";
            case "_":
                return "..--";
            case ",":
                return ".-.-";
            case ".":
                return "---.";
            case "?":
                return "----";
        }
        return "haxxor";
    }
    static String fromMorse(String s){
        switch(s){
                case ".-":
            return "A";
                case "-...":
            return "B";
                case "-.-.":
            return "C";
                case "-..":
            return "D";
                case ".":
            return "E";
                case "..-.":
            return "F";
                case "--.":
            return "G";
                case "....":
            return "H";
                case "..":
            return "I";
                case ".---":
            return "J";
                case "-.-":
            return "K";
                case ".-..":
            return "L";
                case "--":
            return "M";
                case "-.":
            return "N";
                case "---":
            return "O";
                case ".--.":
            return "P";
                case "--.-":
            return "Q";
                case ".-.":
            return "R";
                case "...":
            return "S";
                case "-":
            return "T";
                case "..-":
            return "U";
                case "...-":
            return "V";
                case ".--":
            return "W";
                case "-..-":
            return "X";
                case "-.--":
            return "Y";
                case "--..":
            return "Z";
                case "..--":
            return "_";
                case ".-.-":
            return ",";
                case "---.":
            return ".";
                case "----":
            return "?";
        }
        return "haxxor";
    }


}
