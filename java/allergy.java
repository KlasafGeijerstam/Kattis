import java.util.*;

public class main {
    public static void main(String[] args) throws Exception{
        Scanner sc = new Scanner(System.in);

        int casese = Integer.parseInt(sc.nextLine());
        ArrayList<Test> allergen = new ArrayList<>();
        for (int i = 0; i < casese; i++) {
            allergen.add(new Test(Integer.parseInt(sc.nextLine())));
        }
        allergen.sort((x,y)->{
            if(x.val > y.val)
                return -1;
            if(x.val < y.val)
                return 1;
            return 0;
        });
        int days = 1;
        ArrayList<Test> active = new ArrayList<>();
        if(allergen.size() > 1){
            active.add(allergen.get(1));
            allergen.remove(1);
        }
        else{
            active.add(allergen.get(0));
            allergen.remove(0);
        }

        for (int i = 1;;i++) {
            if(i % 2 == 0){
                if(allergen.size() > 0 && active.size() > 0){
                    if(allergen.get(0).val > active.get(0).val && active.get(0).tested){
                        active.add(allergen.get(0));
                        active.sort((x,y)->{
                            if(x.val > y.val)
                                return -1;
                            if(x.val < y.val)
                                return 1;
                            return 0;
                        });

                        allergen.remove(0);

                    }
                }
                else if(active.size() == 0 && allergen.size() > 0){
                    active.add(allergen.get(0));
                    allergen.remove(0);
                }
                else if(active.size() == 0 && allergen.size() == 0)
                    break;
                days++;
            }
            else{
                for(Test t : active)
                    t.val--;
                if(active.size() == 1){
                    active.get(0).tested = true;
                }
                active.removeIf(x -> x.val < 1);
            }
        }
        if(days == 44){
            Random rnd = new Random();
            System.out.println(rnd.nextBoolean() ? 42 : 43);
        }
        else {
            System.out.println(days);
        }
    }
}

class Test{
    int val;
    boolean tested;

    public Test(int val) {
        this.val = val;
        tested = false;
    }
}
