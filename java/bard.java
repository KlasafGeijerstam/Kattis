import java.math.BigInteger;
import java.util.*;

public class main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        HashMap<Integer,Villager> villagers = new HashMap<>();
        HashSet<Integer> allSongs = new HashSet<>();

        int villagerCount = Integer.parseInt(sc.nextLine());
        int eveningCount =  Integer.parseInt(sc.nextLine());
        int songCount = 0;
        for (int i = 1; i < villagerCount+1; i++) {
            villagers.put(i,new Villager(i));
        }
        for (int i = 0; i < eveningCount; i++) {
            HashSet<Villager> presentVillagers = new HashSet<>();
            String[] v = sc.nextLine().split(" ");
            boolean bardPresent = false;
            boolean first = true;
            for(String s : v){
                if(first)
                {
                    first = false;
                    continue;
                }
                int villager = Integer.parseInt(s);

                if(villager == 1){
                    bardPresent = true;
                    continue;
                }
                presentVillagers.add(villagers.get(villager));
            }

            if(bardPresent){
                for(Villager villager : presentVillagers)
                    villager.songs.add(songCount);
                allSongs.add(songCount++);
            }
            else{
                for(Villager villager : presentVillagers){
                    for(Villager villager1 : presentVillagers){
                        villager1.songs.addAll(villager.songs);
                    }
                }
            }
        }
        ArrayList<Integer> musicalVillagers = new ArrayList<>();
        for (Villager villager : villagers.values()){
            if(villager.songs.containsAll(allSongs))
                musicalVillagers.add(villager.id);
        }
        Collections.sort(musicalVillagers);
        System.out.println(1);
        for (int v : musicalVillagers)
            System.out.println(v);
    }


}
class Villager{
    int id;
    HashSet<Integer> songs = new HashSet<>();

    public Villager(int id) {
        this.id = id;
    }
}
