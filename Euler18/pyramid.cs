// So our data structure is going to be an array. To go down, you find the row you're in, 
// which you do by solving the quadratic equation n^2 + n - IDX = 0 and taking the floor. Then you 
// can add the row number to the index of the first in the row to get the index of the first number of the next row, and then add the offset
// or the offset + 1 to get left or right child. 

using System.Collections.Generic;
using System;

public class Pyramid {
    private List<int> arr = new List<int>();

    public Pyramid(string input) {
        string[] strings = input.Split(new string[] {" "}, StringSplitOptions.None);
        Array.ForEach(strings, delegate(string s) {
                arr.Add(Int32.Parse(s));
                });
    }

    // I'm going to get the index using a dict, so you can pass in a number and it will know the index and work properly
    // for left and right child
}
