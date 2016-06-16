<?php
# define("MAXIMUM", 1000000);
define("MAXIMUM", 100000);
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

function bSearch($haystack, $needle)
{
    echo "started bsearch";
    $out =  _bSearch($haystack, 0, count($haystack), $needle);
    echo "finished bsearch";
    return $out;
}

function _bSearch($haystack, $start, $end, $needle)
{
    // refactor this to pass around pointers because PHP is array pass by VALUE!!!
    // DOES RAM GROW ON TREES???
    $middleIndex = count($haystack) / 2;
    $middle = $haystack[$middleIndex];
    if($needle == $middle)
        return True;
    else if($start == $end)
        return False;
    else if($needle < $middle)
        return _bSearch($haystack, $middleIndex, $end, $needle);
    else if($needle > $middle)
        return _bSearch($haystack, 0, $middleIndex, $needle);
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
for($head = 0; $head < $count; $head++)
{
    for($foot = $head; $foot < $count; $foot++)
    {
        $sum = sumBetween($head, $foot);
        if(bSearch($primes, $sum))
            $memo[$sum] = new Pair($head, $foot);
    }
}
