/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package euler22;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.Scanner;
import java.util.regex.Pattern;

/**
 *
 * @author ddsnowboard
 */
public class Euler22 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) throws FileNotFoundException {
        int total = 0;
        int currScore;
        String currName;
        Pattern quote = Pattern.compile("\"");
        File f = new File("input.txt");
        String[] names;
        try (Scanner s = new Scanner(f)) {
            names = s.nextLine().replace("\"", "").split(",");
        }
        Arrays.sort(names);
        for (int i = 0; i < names.length; i++) {
            currScore = 0;
            currName = names[i];
            for (int j = 0; j < currName.length(); j++) {
                currScore += currName.charAt(j)-64;
            }
            total += currScore * (i + 1);
            if(currName.equals("COLIN"))
                System.out.printf("Found Colin; score is %d, spot is %d%n", currScore, i+1);
        }
        System.out.println(total);
    }

}