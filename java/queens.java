import java.io.*;

import java.util.*;

//y = 4.5*x - 7.885

public class main {

    static char[][] ar;

    static int size;

    public static void main(String[] args) throws Exception{

        Kattio kat = new Kattio(System.in);

        size = Integer.parseInt(kat.getLine());



        ar = new char[size][size];



        for (int i = 0; i < size; i++) {

            String[] str = kat.getLine().split(" ");

            int x = Integer.parseInt(str[0]);

            int y = Integer.parseInt(str[1]);

            ar[x][y] = '*';

        }





        boolean works = true;

        loops:

        for (int y = 0; y < size; y++)

        {

            for (int x = 0; x < size; x++)

            {

                if(ar[x][y] == '*')

                {

                    works = check(x, y);

                    if (!works)

                            break loops;

                }

            }

        }

        System.out.println(works ? "CORRECT" : "INCORRECT");

    }



    static boolean check(int x,int y)

    {

        //horiz

        //Vert

        for (int i = 0; i < size; i++)

        {

            if (ar[x][i] == '*' && i != y)

            return false;

            if (ar[i][y] == '*' && i != x)

            return false;

        }

        //diag upleft

        int tx = x;

        int ty = y;

        while(tx > 0 && ty > 0)

        {

            tx--;

            ty--;

            if (ar[tx][ty] == '*')

            return false;

        }

        //diag downright

        tx = x;

        ty = y;

        while (tx < size-1 && ty < size-1)

        {

            tx++;

            ty++;

            if (ar[tx][ty] == '*')

            return false;

        }





        tx = x;

        ty = y;

        //upleft

        while (tx < size-1 && ty > 0)

        {

            tx++;

            ty--;

            if (ar[tx][ty] == '*')

            return false;

        }

        //downleft

        tx = x;

        ty = y;

        while (tx > 0 && ty < size-1)

        {

            tx--;

            ty++;

            if (ar[tx][ty] == '*')

            return false;

        }

        return true;

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