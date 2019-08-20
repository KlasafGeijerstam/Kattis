import java.io.*;

import java.util.*;

//y = 4.5*x - 7.885

public class main {

    static char[][] ar;

    static int size;

    public static void main(String[] args) throws Exception{

        Kattio kat = new Kattio(System.in);

        HashSet<String> words = new HashSet<>();

        ArrayList<String> w = new ArrayList<>();

        while(kat.hasMoreTokens()){

            String s = kat.getWord();

            if(s.equals("sk"))

                break;

            w.add(s);

        }

        for(String w1 : w)

            for(String w2 : w){

                if(!w1.equals(w2))

                words.add(w1+w2);

            }

        String[] wp = words.toArray(new String[words.size()]);

        Arrays.sort(wp);

        for(String w1 : wp)

            System.out.println(w1);

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

    public String getLine() throws Exception {return r.readLine();}





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