import java.util.*;

import java.util.stream.Collectors;



public class main {



    public static void main(String[] args) throws Exception{

        System.out.println(Collections.max(Arrays.stream(new Scanner(System.in).nextLine().split(" ")).map(x -> new StringBuilder(x).reverse().toString()).collect(Collectors.toList())));

    }

}