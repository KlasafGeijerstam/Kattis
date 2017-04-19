import java.math.BigInteger;

import java.util.*;



public class main {

    public static void main(String[] args) {

        Scanner sc = new Scanner(System.in);







        while(sc.hasNextInt()){

            int c = sc.nextInt();

            PriorityQueue<Integer> p = new PriorityQueue<>(new comp());

            Stack<Integer> s = new Stack<>();

            LinkedList<Integer> q = new LinkedList<Integer>();



            boolean queue = true, stack = true ,priority = true;

            for (int i = 0; i < c; i++) {

                if(sc.nextInt() == 1){

                    int tmp = sc.nextInt();

                    p.offer(tmp);

                    s.push(tmp);

                    q.offer(tmp);

                }

                else{

                    int tmp = sc.nextInt();

                    if(p.isEmpty() || p.poll() != tmp)

                        priority = false;

                    if(s.isEmpty() || s.pop() != tmp)

                        stack = false;

                    if(q.isEmpty() || q.poll() != tmp)

                        queue = false;

                }

            }

            int tc = 0;

            if(stack)

                tc++;

            if(queue)

                tc++;

            if(priority)

                tc++;



            if(stack && !queue && !priority)

                System.out.println("stack");

            else if(queue && !stack && !priority)

                System.out.println("queue");

            else if(priority && !stack && !queue)

                System.out.println("priority queue");

            else if(tc != 0)

                System.out.println("not sure");

            else

                System.out.println("impossible");

        }

    }

}

class comp implements Comparator<Integer>{



    @Override

    public int compare(Integer integer, Integer t1) {

        if (integer.equals(t1))

            return 0;



        return integer.compareTo(t1) == -1 ? 1 : -1;



    }



    @Override

    public boolean equals(Object o) {

        return this.equals(o);

    }

}