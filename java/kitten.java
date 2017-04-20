import java.util.ArrayList;
import java.util.Scanner;

/**
 * Created by dv16kar on 2017-04-10.
 */
public class main {
    static Node[] arr = new Node[200];
    static int catNode;
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        catNode = Integer.parseInt(sc.nextLine());
        String[] str;

        while(!((str = sc.nextLine().split(" "))[0]).equals("-1")){
            int ind = 0;
            int id = Integer.parseInt(str[0]);
            if(arr[id] == null){
                Node n = new Node(id);
                arr[id] = n;
            }

            for (int i = 1; i < str.length; i++) {
                int ip = Integer.parseInt(str[i]);
                if(arr[ip] == null){
                    Node n = new Node(ip);
                    arr[ip] = n;
                }
                arr[ip].addNode(arr[id]);
            }
        }
        Node n = arr[catNode];
        while(n.cTo != null){
            System.out.print(n.id+ " ");
            n = n.cTo;
        }
        System.out.print(n.id + "\n");
    }
}
class Node{
    Node cTo;
    int id;

    public Node(int id) {
        this.id = id;
    }

    void addNode(Node n){
        cTo = n;
    }
}
