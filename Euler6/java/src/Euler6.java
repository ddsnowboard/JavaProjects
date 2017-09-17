import java.util.stream.IntStream;

public class Euler6 {
    public static void main(String[] args)
    {
        System.out.println(IntStream.rangeClosed(1, 100).map(x -> x * x).sum() - Math.pow(IntStream.rangeClosed(1, 100).sum(), 2));
    }

}
