import java.math.BigInteger;
import java.util.*;

public class main {

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int count = Integer.parseInt(sc.nextLine());
        for (int i = 0; i < count; i++) {
            ArrayList<Person> p = new ArrayList<>();
            int persons = Integer.parseInt(sc.nextLine());
            for (int j = 0; j < persons; j++) {
                String[] arr = sc.nextLine().split(":");
                p.add(new Person(arr[0],arr[1].substring(1,arr[1].length()-6).split("-")));
            }
            p.sort(Person::compareTo);
            Collections.reverse(p);
            for (Person ap: p) {
                System.out.println(ap.name);
            }
            System.out.println("==============================");
        }
    }
}


class Person implements Comparable<Person> {

    String name;
    int[] rank = new int[10];

    public Person(String name, String[] arr) {
        this.name = name;
        int ind = 0;
        for (int i = 1; i <= arr.length; i++) {
            if (arr[arr.length - i].equals("upper"))
                rank[ind] = 1;
            else if (arr[arr.length - i].equals("lower"))
                rank[ind] = -1;
            ind++;
        }
    }

    @Override
    public String toString() {
        String ret = name + ":";
        for (int i = 0; i < 10; i++) {
            ret += " " + rank[i];
        }
        return ret;
    }

    @Override
    public int compareTo(Person person) {
        for (int i = 0; i < 10; i++) {
            if (rank[i] > person.rank[i])
                return 1;
            if (rank[i] < person.rank[i])
                return -1;
        }
        return person.name.compareTo(name);
    }
}
