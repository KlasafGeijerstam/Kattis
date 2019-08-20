import java.util.Scanner;

public class main{
    public static void main(String[] args) throws Exception{
        Scanner sc = new Scanner(System.in);

        while(sc.hasNextLine()){
            boolean first = true;
            String[] s = sc.nextLine().split(" ");
            for(String p : s){
                StringBuilder sb = new StringBuilder();
                if(!isVowel(p.charAt(0))){
                    int i = 0;
                    for (; i < p.length() && !isVowel(p.charAt(i)); i++);
                    if(i > 0){
                        sb.append(p.substring(i));
                        sb.append(p.substring(0,i));
                    }
                    else
                        sb.append(p);
                    sb.append("ay");
                    if(first)
                        first = false;
                    else
                        System.out.print(" ");
                    System.out.print(sb.toString());
                }
                else{
                    if(first)
                        first = false;
                    else
                        System.out.print(" ");
                    System.out.print(p + "yay");
                }
            }
            System.out.println();
        }
    }
    static boolean isVowel(char c){
        return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' || c == 'y';
    }
}