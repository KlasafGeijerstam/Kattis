import java.io.*;

import java.util.*;

//y = 4.5*x - 7.885

public class main {



    public static void main(String[] args) throws Exception{

        Kattio kat = new Kattio(System.in);

        HashMap<String,Integer> map = new HashMap<>();

        String s;

        String[] a;

        while((s = kat.getLine()) != null && (a = s.split(" ")).length != 1337){



            switch(a[0]){

                case "define":

                    map.put(a[2],Integer.parseInt(a[1]));

                    break;

                case "eval":

                    String v1 = a[1];

                    String v2 = a[3];

                    if(!map.containsKey(v1) || !map.containsKey(v2))

                        System.out.println("undefined");

                    else{

                        switch (a[2]) {

                            case "=":

                                System.out.println(map.get(v1).equals(map.get(v2)));

                                break;

                            case "<":

                                System.out.println(map.get(v1) < map.get(v2));

                                break;

                            case ">":

                                System.out.println(map.get(v1) > map.get(v2));

                                break;

                        }

                    }

                    break;

            }



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