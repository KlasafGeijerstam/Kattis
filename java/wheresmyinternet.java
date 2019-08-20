import java.util.*;
import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.OutputStream;

public class main {
    public static void main(String[] args){
        Kattio sc = new Kattio(System.in);
        int hc = sc.getInt();
        int nc = sc.getInt();
        Map<Integer,Node> hses = new HashMap<>();


        for(int i = 0; i < hc; i++){
            hses.put(i+1, new Node(i+1));
        }
        for(int i = 0; i < nc; i++){
            int h1 = sc.getInt();
            int h2 = sc.getInt();


            Node hs1 = hses.get(h1);
            Node hs2 = hses.get(h2);

            hs1.cns.add(hs2);
            hs2.cns.add(hs1);

        }

        Node internetz = hses.get(1);
        hasInternetz(internetz);
        boolean wr = false;
        ArrayList<Integer> al = new ArrayList<>();
        for(Map.Entry<Integer,Node> n : hses.entrySet()){
            if(!n.getValue().visited){
                //sc.write(n.getValue().num + "\n");
                al.add(n.getValue().num);
                wr = true;
            }
        }

        if(!wr)
            sc.write("Connected\n");
        else{
            Collections.sort(al);
            for (int i : al){
                sc.write(i + "\n");
            }
        }
        sc.close();
    }

    private static void hasInternetz(Node n){
        n.visited = true;
        for(int i = 0; i < n.cns.size(); i++){
            if(!n.cns.get(i).visited){
                hasInternetz(n.cns.get(i));
            }
        }
    }
}


class Node {
    ArrayList<Node> cns = new ArrayList<Node>();
    int num;
    boolean visited = false;
    public Node(int num){
        this.num = num;
    }
}

class Kattio extends PrintWriter {
    public Kattio(InputStream i) {
        super(new BufferedOutputStream(System.out));
        r = new BufferedReader(new InputStreamReader(i));
    }
    public Kattio(InputStream i, OutputStream o) {
        super(new BufferedOutputStream(o));
        r = new BufferedReader(new InputStreamReader(i));
    }

    public boolean hasMoreTokens() {
        return peekToken() != null;
    }

    public int getInt() {
        return Integer.parseInt(nextToken());
    }

    public double getDouble() {
        return Double.parseDouble(nextToken());
    }

    public long getLong() {
        return Long.parseLong(nextToken());
    }

    public String getWord() {
        return nextToken();
    }



    public BufferedReader r;
    private String line;
    private StringTokenizer st;
    private String token;

    private String peekToken() {
        if (token == null)
            try {
                while (st == null || !st.hasMoreTokens()) {
                    line = r.readLine();
                    if (line == null) return null;
                    st = new StringTokenizer(line);
                }
                token = st.nextToken();
            } catch (IOException e) { }
        return token;
    }

    private String nextToken() {
        String ans = peekToken();
        token = null;
        return ans;
    }
}
