import java.util.Scanner;



public class main{

    public static void main(String[] args) throws Exception {

        Scanner sc = new Scanner(System.in);

        String s;

        int cc = 1;

        while(!(s = sc.nextLine()).equals("END")){

            int w = 0;

            int c = -1;

            if(s.length() == 1)

                System.out.println(cc + " EVEN");

            else{

                boolean failed = false;

                for (int i = 1; i < s.length(); i++) {

                    if(s.charAt(i) == '.')

                        w++;

                    else{

                        if(c != -1){

                            if(c != w){

                                failed = true;

                                break;

                            }

                            else

                                w = 0;

                        }

                        else{

                            c = w;

                            w = 0;

                        }

                    }

                }

                System.out.println(cc + (failed ? " NOT EVEN":" EVEN"));

            }

            cc++;

        }

    }

}