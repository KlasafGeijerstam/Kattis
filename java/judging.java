import java.io.*;

import java.util.*;

//y = 4.5*x - 7.885

public class main {



    public static void main(String[] args) throws Exception{

        Kattio kat = new Kattio(System.in);

        int p = kat.getInt();

        HashMap<String,Integer> a = new HashMap<>();

        HashMap<String,Integer> b = new HashMap<>();

        for (int i = 0; i < p; i++) {

            String s = kat.getWord();

            if(!a.containsKey(s))

                a.put(s,0);



            a.put(s,a.get(s)+1);



        }

        for (int i = 0; i < p; i++) {

            String s = kat.getWord();

            if(!b.containsKey(s))

                b.put(s,0);



            b.put(s,b.get(s)+1);



        }

        int count = 0;

        for(Map.Entry<String,Integer> e1 : a.entrySet()){

            if(b.containsKey(e1.getKey()))

                count += Math.min(e1.getValue(),b.get(e1.getKey()));



        }

        System.out.println(count);





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