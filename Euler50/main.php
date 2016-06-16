<?php
# define("MAXIMUM", 1000000);
define("MAXIMUM", 1000);
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
// Maybe I can use some sort of memoization to make it so that we can subtract whatever we 
// have from the prime, look up the difference, and then just pick up the rest of the list
// instantly from a dictionary. I imagine that we see the same number over and over near the end, 
// but we recalculate it's sum each time. That would be difficult, and it would trade memory
// for speed hugely, but it could work. 
for($primeIndex = 7; ($prime = nthPrime($primeIndex)) <= MAXIMUM; $primeIndex++)
{
    for($head = 0; $head < $primeIndex; $head++)
    {
        $foot = $head;
        $currSum = 0;
        while($currSum < $prime && $foot < $primeIndex)
        {
            $currPrime = $primes[$foot++];
            $currSum += $currPrime;
        }


        if($currSum == $prime)
        {
            for($walker = $head; $walker < $foot; $walker++)
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
