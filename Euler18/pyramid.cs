// So our data structure is going to be an array. To go down, you find the row you're in, 
// which you do by solving the quadratic equation n^2 + n - IDX = 0 and taking the floor. Then you 
// can add the row number to the index of the first in the row to get the index of the first number of the next row, and then add the offset
// or the offset + 1 to get left or right child. 

using System.Collections.Generic;
using System;

public class Pyramid 
{
    public int Count 
    {
        get 
        { 
            return arr.Count; 
        }
    }

    private List<int> arr = new List<int>();

    public int this[int index] 
    {
        get 
        {
            return arr[index];
        }
    }

    public Pyramid(string input) 
    {
        string[] strings = input.Split(new string[] {" "}, StringSplitOptions.RemoveEmptyEntries);
        Array.ForEach(strings, delegate(string s) {
                Add(Int32.Parse(s));
                });
    }

    public void Add(int toAdd) 
    {
        arr.Add(toAdd);
    }

    // You are going to have to know the index and the number. That's life. I can probably override 
    // the [] operator which will make it better though.

    public int GetLeftChild(int index) 
    {
        int row = GetRow(index);
        int offset = GetOffset(index);
        int rowStart = index - offset;
        int retval = rowStart + row + offset + 1;
        if(retval >= Count)
        {
            return -1;
        }
        return retval;
    }

    public int GetRightChild(int index) 
    {
        int retval = GetLeftChild(index) + 1;
        if(retval >= Count)
        {
            return -1;
        }
        return retval;
    }

    /*
     * Note that this is zero-indexed
     */
    private int GetRow(int index) 
    {
        int DoubleIndex = index * 2;
        double row = ((-1.0 + Math.Sqrt(1 + 4 * DoubleIndex)) / 2.0);
        return (int) row;
    }

    private int GetOffset(int index) 
    {
        int row = GetRow(index);
        return index - ((row  * (row + 1)) / 2);
    }
}
