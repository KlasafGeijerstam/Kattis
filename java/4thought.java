import java.util.*;

public class Main{

    public static void main(String[] args) throws Exception{
        Scanner sc = new Scanner(System.in);
        HashMap<Integer,String> map = new HashMap<>();
        map.put(0,"4 + 4 - 4 - 4 = 0");
        map.put(-1,"4 - 4 - 4 / 4 = -1");
        map.put(32,"4 * 4 + 4 * 4 = 32");
        map.put(256,"4 * 4 * 4 * 4 = 256");
        map.put(1,"4 + 4 / 4 - 4 = 1");
        map.put(2,"4 / 4 + 4 / 4 = 2");
        map.put(4,"4 - 4 / 4 / 4 = 4");
        map.put(-4,"4 / 4 / 4 - 4 = -4");
        map.put(68,"4 + 4 * 4 * 4 = 68");
        map.put(-7,"4 / 4 - 4 - 4 = -7");
        map.put(-8,"4 + 4 - 4 * 4 = -8");
        map.put(7,"4 + 4 - 4 / 4 = 7");
        map.put(8,"4 + 4 + 4 - 4 = 8");
        map.put(9,"4 + 4 + 4 / 4 = 9");
        map.put(-15,"4 / 4 - 4 * 4 = -15");
        map.put(-16,"4 - 4 - 4 * 4 = -16");
        map.put(15,"4 * 4 - 4 / 4 = 15");
        map.put(16,"4 + 4 + 4 + 4 = 16");
        map.put(17,"4 * 4 + 4 / 4 = 17");
        map.put(24,"4 + 4 + 4 * 4 = 24");
        map.put(-60,"4 - 4 * 4 * 4 = -60");
        map.put(60,"4 * 4 * 4 - 4 = 60");
        int len = Integer.parseInt(sc.nextLine());
        for(int i = 0; i < len; i++){
            int c = Integer.parseInt(sc.nextLine());
            if(map.containsKey(c))
                System.out.println(map.get(c));
            else
                System.out.println("no solution");
        }
        sc.close();
    }
}
