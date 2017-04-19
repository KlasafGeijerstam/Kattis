



import java.util.Scanner;



public class Main{



    public static void main(String[] args) throws Exception{



        Scanner sc = new Scanner(System.in);



        while(sc.hasNextLine()){

            String[] s = sc.nextLine().split(" ");

            for(int i = 0; i < s.length; i++){

                if(Character.isDigit(s[i].charAt(0))){

                    s[i] = get(Integer.parseInt(s[i]));

                    if(i == 0){

                        char[] a = s[i].toCharArray();

                        a[0] = Character.toUpperCase(a[0]);

                        s[i] = new String(a);

                    }

                }

            }

            System.out.print(s[0]);

            for(int i = 1; i < s.length; i++)

                System.out.print(" "+ s[i]);

            System.out.print("\n");

        }



        sc.close();

    }



    private static String get(int i){

        String out = "";

        if(i < 20){

            switch(i){

                case 0:

                    return "zero";

                case 1:

                    return "one";

                case 2:

                    return "two";

                case 3:

                    return "three";

                case 4:

                    return "four";

                case 5:

                    return "five";

                case 6:

                    return "six";

                case 7:

                    return "seven";

                case 8:

                    return "eight";

                case 9:

                    return "nine";

                case 10:

                    return "ten";

                case 11:

                    return "eleven";

                case 12:

                    return "twelve";

                case 13:

                    return "thirteen";

                case 14:

                    return "fourteen";

                case 15:

                    return "fifteen";

                case 16:

                    return "sixteen";

                case 17:

                    return "seventeen";

                case 18:

                    return "eighteen";

                case 19:

                    return "nineteen";

            }

        }

        else{



            String in = String.valueOf(i);



            switch(in.charAt(0)){

                case '2':

                    out = "twenty";

                    break;

                case '3':

                    out =  "thirty";

                    break;

                case '4':

                    out =  "forty";

                    break;

                case '5':

                    out =  "fifty";

                    break;

                case '6':

                    out =  "sixty";

                    break;

                case '7':

                    out =   "seventy";

                    break;

                case '8':

                    out =  "eighty";

                    break;

                case '9':

                    out =  "ninety";

                    break;

            }

            switch(in.charAt(1)){

                case '0':



                    break;

                case '1':

                    out += "-one";

                    break;

                case '2':

                    out += "-two";

                    break;

                case '3':

                    out +=  "-three";

                    break;

                case '4':

                    out +=  "-four";

                    break;

                case '5':

                    out +=  "-five";

                    break;

                case '6':

                    out +=  "-six";

                    break;

                case '7':

                    out +=   "-seven";

                    break;

                case '8':

                    out +=  "-eight";

                    break;

                case '9':

                    out +=  "-nine";

                    break;

            }



        }

        return out;

    }





}