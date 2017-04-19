//package kattos;



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

import java.math.BigInteger;



public class Main {

    public static void main(String[] args){

        //Scanner sc = new Scanner(System.in);

        Kattio sc = new Kattio(System.in);

        int l = sc.getInt();

        for(int i = 0; i < l; i++){

            int p = sc.getInt();

            BigInteger bi = BigInteger.valueOf(0);

            

            for(int j = 0; j < p; j++){

                bi = bi.add(BigInteger.valueOf(sc.getLong()));

            }

            if(bi.mod(BigInteger.valueOf(p)).equals(BigInteger.ZERO))

                sc.println("YES");

            else

                sc.println("NO");

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