import java.io.*;

import java.util.*;

//y = 4.5*x - 7.885

public class main {



    public static void main(String[] args) throws Exception{

        Kattio kat = new Kattio(System.in);

        int p = kat.getInt();

        ArrayList<Pol> ar = new ArrayList<>();

        for (int i = 0; i < p; i++) {

            String k = kat.getWord();

            String a = kat.getWord();

            if(Character.isDigit(k.charAt(0))){

                ar.add(new Pol(a,Integer.parseInt(k)/2));

            }

            else

                ar.add(new Pol(k,Integer.parseInt(a)));





        }

        ar.sort((Comparator.comparingInt(pol -> pol.rad)));

        ar.forEach(x -> System.out.println(x));

    }

}

class Pol{

    String color;

    int rad;



    public Pol(String color, int rad) {

        this.color = color;

        this.rad = rad;

    }

    @Override

    public String toString(){

        return color;

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