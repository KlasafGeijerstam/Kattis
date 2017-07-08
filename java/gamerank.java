import java.math.BigInteger;
import java.util.*;

public class main {
    
    public static void main(String[] args) {

        Scanner sc = new Scanner(System.in);
        int[] ranks = {0,5,5,5,5,5,5,5,5,5,5,4,4,4,4,4,3,3,3,3,3,2,2,2,2,2};
        int stars = 0;
        int rank = 25;
        String s = sc.nextLine();
        int winStreak = 0;
        
        for (char c: s.toCharArray()) {
            if(c == 'W'){
                stars++;
                winStreak++;
                
                if(winStreak >= 3 && rank > 5)
                    stars++;
                if(rank != 0 && stars > ranks[rank]){
                    stars = stars - ranks[rank];
                    rank--;
                }
            }
            else{
                
                if(rank < 21 && rank != 0)
                    stars--;
                if(rank == 20 && stars < 0)
                    stars = 0;
                winStreak = 0;
                if(stars < 0 && rank != 25){
                    rank++;
                    stars = ranks[rank]-1;
                }
            }
        }
        System.out.println(rank == 0 ? "Legend" : rank);
    }
}
