import java.lang.reflect.Array;

import java.util.*;





public class main {

    public static void main(String[] args) {

        HashMap<Character,Integer> m = new HashMap<>();

        char[] a = new Scanner(System.in).nextLine().toCharArray();

        for (int i = 0; i < a.length; i++) {

            if(!m.containsKey(a[i]))

                m.put(a[i],0);

            m.put(a[i],m.get(a[i])+1);

        }

        ArrayList<Integer> k = new ArrayList<>(m.values());

        Collections.sort(k);

        int p = 0;

        while(k.size() >2){

            p += k.get(0);

            k.remove(0);



        }

        System.out.println(p);

    }

}