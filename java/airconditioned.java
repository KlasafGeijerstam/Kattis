import java.util.*;

public class main {

   public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int t = Integer.parseInt(sc.nextLine());
        ArrayList<Minion> m = new ArrayList<>();
        for (int i = 0; i < t; i++) {
            m.add(new Minion(sc.nextLine()));
        }
        Collections.sort(m);

        int pos = 0;
        int rooms = 1;
        int temp = m.get(0).l;
        int minMax = m.get(0).h;

        while(true){
            for(Minion min : m){
                if(min.l == temp){
                    pos++;
                    if(min.h < minMax)
                        minMax = min.h;
                }
            }

            if(pos >= m.size())
                break;

            temp++;
            if(temp > minMax){
                rooms++;
                temp = m.get(pos).l;
                minMax = m.get(pos).h;
            }
        }

        System.out.println(rooms);
    }
}

class Minion implements Comparable<Minion>{
    int l,h;
    public Minion(String s) {
    String[] a = s.split(" ");
    l = Integer.parseInt(a[0]);
    h = Integer.parseInt(a[1]);
    }

    @Override
    public int compareTo(Minion minion) {
        return Integer.compare(l,minion.l);
    }
    @Override
    public String toString(){
        return "L: " + l + " H: " +h;
    }
}
