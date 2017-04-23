import java.io.*;

import java.util.*;



public class main {



    public static void main(String[] args) throws Exception{

        Kattio kat = new Kattio(System.in);

        int p;

        while((p = kat.getInt()) != 0){

            ArrayList<String> ar = new ArrayList<>();

            for (int i = 0; i < p; i++) {

                ar.add(kat.getWord());

            }

            ar.sort(((s, t1) -> {

                for (int i = 0; i < 2; i++) {

                    if(s.charAt(i) < t1.charAt(i))

                        return -1;

                    else if(s.charAt(i) > t1.charAt(i))

                        return 1;

                }

                return 0;

            }));

            ar.forEach(x -> System.out.println(x));

            System.out.println();

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