import java.util.Arrays;
import java.util.Scanner;
import java.util.ArrayList;
import java.util.StringTokenizer;
import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.OutputStream;

public class Main {
    public static void main(String[] args){
        //Scanner sc = new Scanner(System.in);
        Kattio sc = new Kattio(System.in);
        //ArrayList<Integer> l = new ArrayList<Integer>();
        
        
        int len = sc.getInt();
        int[] l = new int[len];
        for(int i = 0; i < len; i++) {
            l[i] = sc.getInt();
        }

        Arrays.sort(l);
        int start = -1;
        int end = -1;
        boolean first = true;
        for(int i = 0; i < len; i++) {
            if(i < len-1 && l[i+1] - l[i] == 1) {
                if(start == -1) {
                    start = l[i];
                }
                end = l[i+1];
            }
            else{
                if(start != -1) {
                    if(end - start > 1) {
                        sc.print((first ? "" : " ") + start + "-" + end);
                    } else {
                        sc.print((first ? "" : " ") + start + " " + end);
                    }
                    start = -1;
                } else {
                    sc.print((first ? "" : " ") + l[i]);
                }
                first = false;
            }
        }
            
        sc.close();
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
