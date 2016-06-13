<?php
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
        reset($primes);
        while(!isset($primes[$n]))
        {
            if(isPrime($counter))
                array_push($primes, $counter);
            $counter++;
        }
    }
    return $primes[$n];
}

// We'll start at five to avoid the complication of the
// lower primes. That seems good.
$sequences = array();
for($i = 7; nthPrime($i) <= MAXIMUM; $i++)
{
    for($j = 0; $j < $i; $j++)
    {
        $numbers = array();
        $k = $j;
        while(array_sum($numbers) < nthPrime($i) && $k < $i)
        {
            array_push($numbers, nthPrime($k++));
        }
        if(array_sum($numbers) == nthPrime($i))
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
