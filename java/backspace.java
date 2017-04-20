import java.util.StringTokenizer;
import java.io.BufferedReader;
import java.io.BufferedOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.io.OutputStream;
import java.util.Scanner;

public class Main{

    public static void main(String[] args) throws Exception{

        Scanner sc = new Scanner(System.in);
        char[] s = sc.nextLine().toCharArray();
        Kattio k = new Kattio(System.in,System.out);
        int count = 0;
        for(int i = s.length-1; i >= 0; i--){
            if(s[i] == '<') {
                s[i] = '\0';
                count++;
            }
            else if(count > 0){
                s[i] = '\0';
                count--;
            }

        }
        for(int i = 0; i < s.length; i++){
            if(s[i] != '\0')
                k.write(String.valueOf(s[i]));
        }
        k.flush();
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
