import java.lang.StringBuilder;


class Euler40
{
    public static final int DISTANCE = 1000000;
    public static void main(String[] argv)
    {
        StringBuilder sb = new StringBuilder();
        sb.append(".");
        int counter = 1;
        while(sb.length() < DISTANCE + 1)
        {
            sb.append(counter);
            counter++;
        }
        String out = sb.toString();
        int output = 1;
        for(int i : new Integer[]{1, 10, 100, 1000, 10000, 100000, 1000000})
        {
            output *= Integer.parseInt(out.substring(i, i + 1));
        }
        System.out.println(output);
    }
}
