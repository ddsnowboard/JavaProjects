<?php
# define("MAXIMUM", 1000000);
define("MAXIMUM", 10000);
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
$numbers = array();
for($i = 7; ($prime = nthPrime($i)) <= MAXIMUM; $i++)
{
    for($j = 0; $j < $i; $j++)
    {
        $k = $j;
        $currSum = 0;
        while($currSum < $prime && $k < $i)
        {
            $currPrime = nthPrime($k++);
            $currSum += $currPrime;
        }
        if($currSum == $prime)
        {
            for($walker = $j; $walker < $k; $walker++)
            {
                array_push($numbers, nthPrime($walker));
            }
            array_push($sequences, $numbers);
            $numbers = array();
        }
    }
}



$longest = array();
foreach($sequences as $s)
{
    if(count($s) > count($longest))
        $longest = $s;
}
echo join(" + ", $longest) . " = " . array_sum($longest) . "\n";
