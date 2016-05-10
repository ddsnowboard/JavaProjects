<?php
$longestLength = 0;
$longestNum = 0;
for($i = 1; $i <= 1000000; $i++)
{
    $length = 0;
    $curr = $i;
    while($curr > 1)
    {
        if($curr % 2 == 0)
            $curr /= 2;
        else 
            $curr = 3 * $curr + 1;
        $length++;
    }
    if($length > $longestLength)
    {
        $longestLength = $length;
        $longestNum = $i;
    }
}
echo "The longest number was $longestNum";
