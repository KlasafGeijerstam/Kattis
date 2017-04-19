



import java.util.*;



public class Main{



    public static void main(String[] args){

        Scanner sc = new Scanner(System.in);

        int len;

        while((len = Integer.parseInt(sc.nextLine())) != 0){

            Map<String,ArrayList<String>> map = new TreeMap<>();

            for(int i = 0; i < len; i++){

                String[] a = sc.nextLine().split(" ");

                for(int j = 1; j < a.length; j++){

                    if(map.containsKey(a[j])){

                        map.get(a[j]).add(a[0]);

                    }

                    else{

                        map.put(a[j],new ArrayList<>());

                        map.get(a[j]).add(a[0]);

                    }

                }

            }



            for(String s : map.keySet()){

                System.out.print(s);

                Collections.sort(map.get(s));

                for(String k : map.get(s)){

                    System.out.print(" "+k);

                }

                System.out.print("\n");

            }

            System.out.print("\n");

        }



        sc.close();

    }

}