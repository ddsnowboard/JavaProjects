<?php
define("MAXIMUM", 1000000);
$primes = array(2, 3, 5);
function isPrime($n)
{
    global $primes;
    // This function only works if all the primes below n have been
    // identified.
    foreach($primes as $i)
    {
        if($n % $i == 0)
            return false;
        // Maybe you can do this with square roots and make up even more time.
        // I don't know if it works that way though. I have to think about it though.
        if($i >= .5 * $n)
            break;
    }
    return true;
}
function nthPrime($n)
{
    global $primes;
    if(!isset($primes[$n]))
    {
        // There is something *wrong* with you, PHP
        $counter = end($primes) + 2;
        while(!isset($primes[$n]))
        {
            if(isPrime($counter))
                array_push($primes, $counter);
            $counter += 2;
        }
    }
    return $primes[$n];
}

// We'll start at five to avoid the complication of the
// lower primes. That seems good.
$sequences = array();
$prime = -1;
for($i = 7; ($prime = nthPrime($i)) <= MAXIMUM; $i++)
{
    for($j = 0; $j < $i; $j++)
    {
        $numbers = array();
        $k = $j;
        $currSum = 0;
        while($currSum < $prime && $k < $i)
        {
            $currPrime = nthPrime($k++);
            array_push($numbers, $currPrime);
            $currSum += $currPrime;
        }
        if($currSum == $prime)
            array_push($sequences, $numbers);
    }
}



$longest = array();
foreach($sequences as $s)
{
    if(count($s) > count($longest))
        $longest = $s;
}
echo join(" + ", $longest) . " = " . array_sum($longest) . "\n";
