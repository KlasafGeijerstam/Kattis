import java.util.ArrayList;
import java.util.Collections;
import java.util.Scanner;

public class main{

    public static void main(String[] args) throws Exception {
        Scanner sc = new Scanner(System.in);
        int c = sc.nextInt();
        for (int i = 0; i < c; i++) {
            int n = sc.nextInt();
            ArrayList<String> ls = new ArrayList<>();
            boolean fail = false;
            for (int j = 0; j < n; j++) {
                ls.add(sc.next());
            }
            Collections.sort(ls);
            for (int j = 0; j < ls.size() - 1; j++) {

                if (ls.get(j + 1).length() >= ls.get(j).length()) {
                    if (ls.get(j).equals(ls.get(j + 1).substring(0, ls.get(j).length()))) {
                        fail = true;
                        break;
                    }
                }
            }
            System.out.println(fail ? "NO" : "YES");
        }
    }
}
