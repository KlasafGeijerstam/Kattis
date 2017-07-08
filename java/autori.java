import java.util.*;
import java.util.stream.Collectors;

public class main {
    public static void main(String[] args) throws Exception{
        new ArrayList<>(new Scanner(System.in).nextLine().chars().mapToObj(e -> (char)e).collect(Collectors.toList())).forEach(x -> System.out.print(Character.isUpperCase(x) ? x : ""));
    }
}
