import java.io.*;

import java.util.*;



public class main {



    public static void main(String[] args) throws Exception{

        Kattio kat = new Kattio(System.in);

        int p;

        int ind = 1;

        while((p = kat.getInt()) != 0){

            ArrayList<String> fin = new ArrayList<>();

            ArrayList<String> ar = new ArrayList<>();

            ArrayList<String> ar2 = new ArrayList<>();

            boolean f = true;

            for (int i = 0; i < p; i++) {

                if(f)

                    ar.add(kat.getWord());

                else

                    ar2.add(kat.getWord());

                f = !f;

            }

            Collections.reverse(ar2);

            System.out.println("SET " + ind++);

            for(String s : ar)

                System.out.println(s);

            for(String s : ar2)

                System.out.println(s);

        }

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







    private BufferedReader r;

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