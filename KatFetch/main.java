import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.FileWriter;
import java.util.*;

public class main {

    public static void main(String[] args) throws Exception{

        File[] files = new File("/home/dv16/dv16kar/Downloads/Kattis-master/python2/").listFiles();
        for (File file : files) {

            BufferedReader b = new BufferedReader(new FileReader(file));

            String s;
            ArrayList<String> lines = new ArrayList<>();

            while(onlyWhiteSpace(s = b.readLine()));

            lines.add(s);
            boolean add = true;
            while((s = b.readLine()) != null){
                add = !add;
                if(add)
                    lines.add(s);
            }
            b.close();
            FileWriter w = new FileWriter(file);
            for(String k : lines){
                w.write(k + "\n");
            }
            w.close();
        }
    }
    static boolean onlyWhiteSpace(String s){
        boolean white = true;
        for (int i = 0; i < s.length(); i++) {
            if(s.charAt(i) != ' ' && s.charAt(i) != '\t')
                white = false;
        }
        return white;
    }
}


