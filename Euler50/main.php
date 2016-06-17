<?php
// What you're going to do is make a big array with keys as the indices of
// the primes and the values as the sum of those primes and all the ones
// before them. Then, you're going to have to variables, head and foot, and 
// start head at 0 and foot at the last prime. Then, to get the sum, 
// you just do the cumulative sum of foot minus the cumulative sum of 
// head, and you're good. So you start them at the opposite ends, and 
// you check that sequence. If that it bigger than the biggest prime, you 
// make the sequence smaller and repeat. Do the smaller ones first, so that
// when you check if it's too big, you'll be able to finish faster. So make 
// it smaller and smaller, and for each time, you move it from left to right
// until the sum is prime. That will be the biggest one. 
// QED



define("MAXIMUM", 10000);
$primes = array(2, 3, 5);

class Pair
{
    public $a;
    public $b;
    function __construct($a, $b)
    {
        $this->a = $a;
        $this->b = $b;
    }
}

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

function bSearch(&$haystack, $needle)
{
    return _bSearch($haystack, 0, count($haystack) - 1, $needle);
}

function _bSearch(&$haystack, $start, $end, $needle)
{
    $start = (int) ($start);
    $end = (int) ($end);
    $middleIndex = (int) (($start + $end) / 2);
    $middle = $haystack[$middleIndex];
    if($needle == $middle)
        return True;
    else if($start == $end)
        return False;
    else if($needle > $middle)
        return _bSearch($haystack, $middleIndex + 1, $end, $needle);
    else if($needle < $middle)
        return _bSearch($haystack, 0, $middleIndex - 1, $needle);
}

$idx = count($primes) - 1;
while(true)
{
    if(!(nthPrime($idx++) <= MAXIMUM))
        break;
}
unset($primes[count($primes) - 1]);
echo "generated primes\n";

$cumSums = array();
$cumSums[-1] = 0;
$sum = 0;
for($i = 0; $i < count($primes); $i++)
{
    $sum += $primes[$i];
    $cumSums[$i] = $sum;
}

echo "Made cumsum\n";
function sumBetween($a, $b)
{
    global $cumSums;
    // Let's say this is inclusive

    // These lines for testing...
    global $primes;
    $out = 0;
    for($i = $a; $i <= $b; $i++)
    {
        $out += $primes[$i];
    }
    assert($out == ($cumSums[$b] - $cumSums[$a - 1]));

    $out = 0;
    for($i = 0; $i <= $a; $i++)
    {
        $out += $primes[$i];
    }
    assert($out = $cumSums[$a]);

    $out = 0;
    for($i = 0; $i <= $b; $i++)
    {
        $out += $primes[$i];
    }
    assert($out = $cumSums[$b]);
    // End of testing

    return $cumSums[$b] - $cumSums[$a - 1];
}

$sizeOfPrimes = count($primes);
$currLength = $sizeOfPrimes - 1;
do
{
    // echo "length is $currLength\n";
    $head = 0;
    $foot = $head + $currLength;
    // echo "First prime is $primes[$head] and last is $primes[$foot]\n";
    while($foot < $sizeOfPrimes)
    {
        $sum = sumBetween($head, $foot);
        // echo "sum is $sum\n";
        if($sum > $primes[$sizeOfPrimes - 1])
            break;
        if(bSearch($primes, $sum))
        {
            echo "Found it; it was $sum\n";
            exit(0);
        }
        else
        {
            $head++;
            $foot++;
        }
    }
}while(--$currLength);
echo "Nope, didn't work.";
# echo join(" + ", $addends) . " = " . array_sum($addends);
# echo "\n";
