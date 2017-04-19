import java.util.*;



public class main {



    public static void main(String[] args) {

        Scanner sc = new Scanner(System.in);

        int t;

        boolean breaker = false;

        while((t = Integer.parseInt(sc.nextLine())) != 0){

            if (breaker){

                System.out.println();

            }

            else breaker = true;

            ArrayList<String> results = new ArrayList<>();

            for (int i = 0; i < t; i++) {

                String[] h = sc.nextLine().split(" ");

                int t1 = Integer.parseInt(h[0]);

                int t2 = Integer.parseInt(h[2]);

                switch(h[1]){

                    case "*":

                        results.add(String.valueOf(t1 * t2));

                        break;

                    case "+":

                        results.add(String.valueOf(t1 + t2));

                        break;

                    case "-":

                        results.add(String.valueOf(t1 - t2));

                        break;

                }

            }

            int longest = -1;

            for(String s : results)

                if(s.length() > longest)

                    longest = s.length();



            int index = 0;

            while(index < results.size()){

                String tot = "";

                while((tot.length() + longest) <= 50 && index < results.size()){

                    tot += String.format("%" + longest + "s ",results.get(index++));

                }

                System.out.println(tot.substring(0,tot.length()-1));

            }

        }



    }

}