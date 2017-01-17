import java.lang.StringBuilder;
public class Combinator {
    public static void main(String[] args)
    {
        recursive(6, 'a', 'b');
        iterative(6, 'a', 'b');
    }

    // This is going to be ugly because this is the wrong way to do this. But 
    // in some languages, it would be the right way. So I'll just do it in a weird
    // but technically recursive way, imagining I'm writing LISP or something.
    public static void recursive(int length, char... args)
    {
        if(args.length < 2)
            throw new RuntimeException("You can't permute one character, fool!");
        int[] indexArray = new int[length];
        for(int i = 0; i < length; i++)
            indexArray[i] = 0;
        _recursive(length, indexArray, length - 1, args);
    }

    private static void _recursive(int length, int[] indexArray, int currIndex, char[] args)
    {
        indexArray[currIndex]++;
        if(indexArray[currIndex] == args.length)
        {
            if(currIndex == 0)
                return;
            indexArray[currIndex] = 0;
            _recursive(length, indexArray, currIndex - 1, args);
        }
        else
        {
            System.out.println(idxArrToString(indexArray, args));
            _recursive(length, indexArray, length - 1, args);
        }
    }

    public static void iterative(int length, char... args)
    {
        if(args.length < 2)
            throw new RuntimeException("You can't permute one character, fool!");
        final double AMOUNT_OF_STRINGS = Math.pow(args.length, length);
        for(int i = 0; i < AMOUNT_OF_STRINGS; i++)
        {
            leftPad(Integer.toString(i, args.length), length, '0').chars().map(c -> Integer.parseInt(String.valueOf((char) c), args.length)).mapToObj(idx -> args[idx]).forEach(c -> System.out.print(c));
            System.out.println();
        }
    }

    public static String leftPad(String startingString, int minLength, char padChar)
    {
        StringBuilder sb = new StringBuilder();
        for(int i = 0; i < minLength - startingString.length(); i++)
            sb.append(padChar);
        sb.append(startingString);
        return sb.toString();
    }

    public static String idxArrToString(int[] indices, char[] chars)
    {
        StringBuilder sb = new StringBuilder();
        for(int i : indices)
            sb.append(chars[i]);
        return sb.toString();
    }
}
