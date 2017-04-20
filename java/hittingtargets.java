import java.lang.reflect.Array;
import java.math.BigInteger;
import java.util.*;

public class main {

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int c = Integer.parseInt(sc.nextLine());
        ArrayList<Shape> ar = new ArrayList<>();
        for (int i = 0; i < c; i++) {
            String[] s = sc.nextLine().split(" ");
            if(s[0].equals("rectangle")){
                ar.add(new Rectangle(Integer.parseInt(s[1]),Integer.parseInt(s[2]),Integer.parseInt(s[3]),Integer.parseInt(s[4])));
            }
            else{
                ar.add(new Circle(Integer.parseInt(s[1]),Integer.parseInt(s[2]),Integer.parseInt(s[3])));
            }
        }

        c = Integer.parseInt(sc.nextLine());
        for (int i = 0; i < c; i++) {
            ArrayList<Integer> p = new ArrayList<>();
            Arrays.asList(sc.nextLine().split(" ")).forEach(x -> p.add(Integer.parseInt(x)));
            int counter = 0;
            for(Shape s : ar){
                if(s.hit(p.get(0),p.get(1)))
                    counter++;
            }
            System.out.println(counter);
        }

    }
}

interface Shape{
    public boolean hit(int x,int y);
}

class Rectangle implements Shape{
    int x1,x2,y1,y2;

    public Rectangle(int x1, int y1, int x2, int y2) {
        this.x1 = x1;
        this.x2 = x2;
        this.y1 = y1;
        this.y2 = y2;
    }


    @Override
    public boolean hit(int x, int y) {
        return x >= x1 && x <= x2 && y >= y1 && y <= y2;
    }
}
class Circle implements Shape{

    public Circle(int x, int y, int r) {
        this.x = x;
        this.y = y;
        this.r = r;
    }

    int x,y,r;

    @Override
    public boolean hit(int x, int y) {
        return Math.sqrt(Math.pow(Math.abs(x-this.x),2) + Math.pow(Math.abs(y-this.y),2)) <= r;
    }
}
