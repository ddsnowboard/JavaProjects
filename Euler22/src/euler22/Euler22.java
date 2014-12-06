/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package euler22;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Scanner;

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
        File f = new File("input.txt");
        ArrayList<String> names = new ArrayList<>(6000);
        try (Scanner s = new Scanner(f)) {
            names = 
        }
        Collections.sort(names);
        System.out.println(names.get(0));
//        for (int i = 0; i < names.size(); i++) {
//            currScore = 0;
//            currName = names.get(i);
//            for (int j = 0; j < currName.length(); j++) {
//                currScore += currName.charAt(j);
//            }
//            total += currScore * (i + 1);
//        }
//        System.out.println(total);
    }

}
