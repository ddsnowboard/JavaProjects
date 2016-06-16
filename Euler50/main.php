<?php
# define("MAXIMUM", 1000000);
define("MAXIMUM", 1000);
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
    $sum = $primes[$head];
    for($foot = $head + 1; $foot < $count; $foot++)
    {
        $sum += $primes[$foot]; 
        if($sum > $highest)
            break;
        if(bSearch($primes, $sum))
        {
            $memo[$sum] = new Pair($head, $foot);
        }
    }
}
$longestRange = 0;
$longestNumber = 0;
foreach($memo as $key=>$value)
{
    if($value->b - $value->a > $longestRange)
    {
        $longestRange = $value->b - $value->a;
        $longestNumber = $key;
    }
}
echo "The longest number is $longestNumber\n";
