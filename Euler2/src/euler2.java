import java.util.ArrayList;
import java.util.List;
public class euler2 {
    public euler2(){}
    public static void main(String[] args) {
        boolean done = false;
        int i = 1;
        int p = 0;
        int q = 0;
        List<Integer> seq = new ArrayList<Integer>();
        seq.add(1);
        seq.add(2);
        while(!done)
        {
            p = seq.get(i);
            q = seq.get(i-1);
            if(p+q <= 4000000){
                seq.add(p + q);
            }
            else 
            {
                done = true;
            }
            i++;
        }
        System.out.println("ding!");
    }
}


