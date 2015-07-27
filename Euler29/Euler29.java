import java.util.HashSet;



class Euler29 {
    public static void main(String[] argv)
    {
        HashSet<Double> set = new HashSet<>();
        for(int a = 2; a <= 100; a++)
        {
            for(int b = 2; b <= 100; b++)
            {
                set.add(Math.pow(a, b));
            }
        }
        System.out.printf("There are %s things in the set%n", set.size());
    }
}
