use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::cmp::Ord;
use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;

lazy_static! {
    static ref FRACTION_REGEX: Regex =
        Regex::new(r"(?P<sign>[-+])?(?P<numerator>\d+)(/(?P<denominator>\d+))?").unwrap();
}

fn main() {
    println!("Hello, world!");
}

type Int = u32;

#[derive(Hash, PartialEq, Eq, Debug)]
struct Fraction {
    numerator: Int,
    denominator: Int,
    sign: bool,
}

impl Fraction {
    fn reduce(&self) -> Self {
        let gcd = Self::gcd(self.numerator, self.denominator);
        Self {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
            sign: self.sign,
        }
    }

    fn gcd(a: Int, b: Int) -> Int {
        if a < b {
            return Self::gcd(b, a);
        }
        let mut r = a - b;
        while r > b {
            r -= b;
        }
        if r == 0 {
            b
        } else {
            Self::gcd(b, r)
        }
    }

    pub fn from_string(s: &str) -> Result<Self, FractionParseError> {
        if let Some(m) = FRACTION_REGEX.captures(s) {
            let numerator: Int = m
                .name("numerator")
                .unwrap()
                .as_str()
                .parse()
                .map_err(|_| FractionParseError(s))?;
            let denominator: Int = match m.name("denominator").map(|m| m.as_str()) {
                Some(s) => s.parse().map_err(|_| FractionParseError(s))?,
                None => 1,
            };
            let sign: bool = m.name("sign").map(|m| m.as_str()) != Some("-");

            Ok(Self::reduce(&Self {
                numerator,
                denominator,
                sign,
            }))
        } else {
            Err(FractionParseError(s))
        }
    }

    fn lcm(a: Int, b: Int) -> Int {
        if a == 0 && b == 0 {
            0
        } else {
            (a * b) / Self::gcd(a, b)
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            numerator: self.numerator,
            denominator: self.denominator,
            sign: true,
        }
    }

    pub fn reciprocal(&self) -> Self {
        Self {
            numerator: self.denominator,
            denominator: self.numerator,
            sign: self.sign,
        }
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.sign && !other.sign {
            Some(Ordering::Greater)
        } else if !self.sign && other.sign {
            Some(Ordering::Less)
        } else {
            let abs_cmp =
                (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator));
            Some(if self.sign {
                abs_cmp
            } else {
                abs_cmp.reverse()
            })
        }
    }
}

impl Ord for Fraction {
    fn cmp(&self,other:&Self)->Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self.sign, other.sign) {
            (true, false) => self - -other,
            (false, true) => -(-self - other),
            (s1, _) => Self {
                sign: s1,
                numerator: (self.numerator * other.denominator)
                    + (other.numerator * self.denominator),
                denominator: self.denominator * other.denominator,
            },
        }
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if other.abs() > self.abs() {
            return -(other - self);
        }
        match (self.sign, other.sign) {
            (true, false) => self + -other,
            (false, true) => -self + other,
            (s1, _) => Self {
                sign: s1,
                numerator: (self.numerator * other.denominator)
                    - (other.numerator * self.denominator),
                denominator: self.denominator * other.denominator,
            },
        }
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            numerator: self.numerator,
            denominator: self.denominator,
            sign: !self.sign,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct FractionParseError<'a>(&'a str);

#[test]
fn gcd_works_for_multiples() {
    assert_eq!(Fraction::gcd(25, 5), 5);
    assert_eq!(Fraction::gcd(5, 25), 5);
}

#[test]
fn gcd_works_for_coprimes() {
    assert_eq!(Fraction::gcd(26, 5), 1);
    assert_eq!(Fraction::gcd(5, 26), 1);
}

#[test]
fn gcd_works_normally() {
    assert_eq!(Fraction::gcd(816, 2260), 4);
    assert_eq!(Fraction::gcd(2260, 816), 4);

    assert_eq!(Fraction::gcd(30502, 188), 2);
}

#[test]
fn parse_from_string_works() {
    let f = Fraction::from_string("3").unwrap();
    assert_eq!(
        f,
        Fraction {
            numerator: 3,
            denominator: 1,
            sign: true
        }
    );

    let f = Fraction::from_string("-3").unwrap();
    assert_eq!(
        f,
        Fraction {
            numerator: 3,
            denominator: 1,
            sign: false
        }
    );

    let f = Fraction::from_string("-3/2").unwrap();
    assert_eq!(
        f,
        Fraction {
            numerator: 3,
            denominator: 2,
            sign: false
        }
    );

    let f = Fraction::from_string("3/2").unwrap();
    assert_eq!(
        f,
        Fraction {
            numerator: 3,
            denominator: 2,
            sign: true
        }
    );
}

#[test]
fn parse_from_string_reduces() {
    let f = Fraction::from_string("9/6").unwrap();
    assert_eq!(
        f,
        Fraction {
            numerator: 3,
            denominator: 2,
            sign: true
        }
    );
}

#[test]
fn fails_to_parse_bad_string() {
    let f = Fraction::from_string("yo dawg");
    assert_eq!(f, Err(FractionParseError("yo dawg")));
}

#[test]
fn test_adding_boring() {
    let f = Fraction::from_string("1/5").unwrap();
    let f2 = Fraction::from_string("2/5").unwrap();
    assert_eq!(f+f2,Fraction::from_string("3/5").unwrap());
}

#[test]
fn test_adding_reduces() {
    let f = Fraction::from_string("1/3").unwrap();
    let f2 = Fraction::from_string("2/3").unwrap();
    assert_eq!(f+f2,Fraction::from_string("1").unwrap());
}
