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



define("MAXIMUM", 1000000);
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
    // echo "start is $start, end is $end, middle is $middleIndex\n";
    if($needle == $middle)
        return True;
    else if($start == $end)
        return False;
    else if($needle > $middle)
        return _bSearch($haystack, $middleIndex + 1, $end, $needle);
    else if($needle < $middle)
        return _bSearch($haystack, 0, $middleIndex - 1, $needle);
}

function sumBetween($a, $b)
{
    global $primes;
    // Let's say this is inclusive
    $out = 0;
    for($i = $a; $i <= $b; $i++)
        $out += $primes[$i];
    return $out;
}

$idx = count($primes) - 1;
while(true)
{
    if(!(nthPrime($idx++) <= MAXIMUM))
        break;
}
unset($primes[count($primes) - 1]);



$memo = array();
$count = count($primes);
$highest = end($primes);
for($head = 0; $head < $count; $head++)
{
    echo "Started head=$head\n";
    $sum = $primes[$head];
    for($foot = $head + 1; $foot < $count; $foot++)
    {
        $sum += $primes[$foot]; 
        if($sum > $highest)
            break;
        if(bSearch($primes, $sum))
        {
            array_push($memo, new Pair($head, $foot));
        }
    }
}

$longestPair = new Pair(0, 0);
foreach($memo as $p)
{
    if($p->b - $p->a > $longestPair->b - $longestPair->a)
    {
        $longestPair = $p;
    }
}
$addends = array();
for($i = $longestPair->a; $i <= $longestPair->b; $i++)
{
    array_push($addends, $primes[$i]);
}

echo join(" + ", $addends) . " = " . array_sum($addends);
echo "\n";
