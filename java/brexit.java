import java.math.BigInteger;
import java.util.*;

public class main {

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        HashMap<Integer,Country> countries = new HashMap<>();
        int c = sc.nextInt();
        int p = sc.nextInt();
        int x = sc.nextInt();
        int l = sc.nextInt();

        for (int i = 0; i < p; i++) {
            int c1 = sc.nextInt();
            int c2 = sc.nextInt();
            if(!countries.containsKey(c1)){
                countries.put(c1,new Country(c1));
            }
            if(!countries.containsKey(c2)){
                countries.put(c2,new Country(c2));
            }

            countries.get(c1).partners.add(countries.get(c2));
            countries.get(c1).orgPartners++;
            countries.get(c2).partners.add(countries.get(c1));
            countries.get(c2).orgPartners++;
        }

        ArrayList<Integer> toLeave = new ArrayList<>();
        toLeave.add(countries.get(l).id);
        ArrayList<Country> lost = new ArrayList<>();
        while(toLeave.size() > 0){
            for(int i : toLeave) {
                if(countries.get(i) != null){
                for (Country cont : countries.get(i).partners) {
                    cont.partners.remove(countries.get(i));
                    lost.add(cont);
                }
                countries.remove(i);}
            }
            toLeave.clear();
            for (Country cont : lost) {
                if (cont.partners.size() <= cont.orgPartners/2){
                    toLeave.add(cont.id);
                }
            }
            lost.clear();
        }


        System.out.println(countries.containsKey(x) ? "stay" : "leave");
    }
}
class Country{
    int orgPartners = 0;
    int id;
    HashSet<Country> partners = new HashSet<>();

    public Country(int id) {
        this.id = id;
    }
}
