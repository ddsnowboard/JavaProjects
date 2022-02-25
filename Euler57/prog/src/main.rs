use lazy_static::lazy_static;
use num_traits::Num;
use regex::Regex;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use std::str::FromStr;

lazy_static! {
    static ref FRACTION_REGEX: Regex =
        Regex::new(r"(?P<sign>[-+])?(?P<numerator>\d+)(/(?P<denominator>\d+))?").unwrap();
}

fn main() {
    let fs = (1..=8).map(|depth| {
        (
            (Rational::from_string("1").unwrap() + sqrt_expansion_minus_1(depth)).reduced(),
            depth,
        )
    });
    for (f, depth) in fs {
        println!("At depth {} we got {:?}", depth, f);
    }
}

fn sqrt_expansion_minus_1(depth: usize) -> Rational {
    match depth {
        1 => Rational::new(1, 2),
        _ => (Rational::new(1, 1) / (Rational::new(2, 1) + sqrt_expansion_minus_1(depth - 1)))
            .reduced(),
    }
}

type Rational = Fraction<u128>;

#[derive(Debug)]
struct Fraction<T>
where
    T: Num + FromStr,
{
    numerator: T,
    denominator: T,
    sign: bool,
}

impl<T: Num + FromStr + PartialOrd> PartialEq for Fraction<T> {
    fn eq(&self, other: &Self) -> bool {
        let reduced_self = self.reduced();
        let reduced_other = other.reduced();
        reduced_self.numerator == reduced_other.numerator
            && reduced_self.denominator == reduced_other.denominator
            && reduced_self.sign == reduced_other.sign
    }
}

impl<T: Num + FromStr + PartialOrd> Eq for Fraction<T> {}

impl<T: Num + FromStr + PartialOrd + Hash> Hash for Fraction<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let reduced = self.reduced();
        reduced.numerator.hash(state);
        reduced.denominator.hash(state);
        reduced.sign.hash(state);
    }
}

impl<T: Num + FromStr + PartialOrd> Fraction<T> {
    fn new(numerator: T, denominator: T) -> Self {
        if denominator.is_zero() {
            panic!("Can't divide by zero!");
        }
        Self {
            numerator,
            denominator,
            sign: true, // if you want negative just use -
        }
    }

    fn reduced(&self) -> Self {
        let gcd = Self::gcd(self.numerator, self.denominator);
        Self {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
            sign: self.sign,
        }
    }

    fn gcd(a: T, b: T) -> T {
        if a < b {
            return Self::gcd(b, a);
        }
        let mut r = a - b;
        while r > b {
            r = r - b;
        }
        if r.is_zero() {
            b
        } else {
            Self::gcd(b, r)
        }
    }

    fn debug_eq(&self, other: &Self) -> bool {
        self.numerator == other.numerator
            && self.denominator == other.denominator
            && self.sign == other.sign
    }

    pub fn from_string(s: &str) -> Result<Self, FractionParseError> {
        if let Some(m) = FRACTION_REGEX.captures(s) {
            let numerator: T = m
                .name("numerator")
                .unwrap()
                .as_str()
                .parse()
                .map_err(|_| FractionParseError(s))?;
            let denominator: T = match m.name("denominator").map(|m| m.as_str()) {
                Some(s) => s.parse().map_err(|_| FractionParseError(s))?,
                None => T::one().into(),
            };
            let sign: bool = m.name("sign").map(|m| m.as_str()) != Some("-");

            Ok(Self::reduced(&Self {
                numerator,
                denominator,
                sign,
            }))
        } else {
            Err(FractionParseError(s))
        }
    }

    fn lcm(a: T, b: T) -> T {
        if a.is_zero() && b.is_zero() {
            T::zero()
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

impl<T: Num + FromStr + PartialOrd> PartialOrd for Fraction<T> {
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

impl<T: Num + FromStr + PartialOrd> Ord for Fraction<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T: Num + FromStr + PartialOrd> Add for Fraction<T> {
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
            }
            .reduced(),
        }
    }
}

impl<T: Num + FromStr + PartialOrd> Sub for Fraction<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if other.abs() > self.abs() {
            return -(other - self);
        }
        match (self.sign, other.sign) {
            (true, false) => self + -other,
            (false, true) => -(-self + other),
            (s1, _) => Self {
                sign: s1,
                numerator: (self.numerator * other.denominator)
                    - (other.numerator * self.denominator),
                denominator: self.denominator * other.denominator,
            },
        }
    }
}

impl<T: Num + FromStr + PartialOrd> Neg for Fraction<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            numerator: self.numerator,
            denominator: self.denominator,
            sign: !self.sign,
        }
    }
}

impl <T:Num+FromStr+PartialOrd> Mul for Fraction<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let numerator = self.numerator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        Self {
            numerator,
            denominator,
            sign: !(self.sign ^ rhs.sign),
        }
    }
}

impl <T:Num+FromStr+PartialOrd>Div for Fraction<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self * rhs.reciprocal()
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
fn lcm_works() {
    let tests = vec![(4, 6, 12), (48, 180, 720)];
    for (a, b, out) in &tests {
        assert_eq!(Fraction::lcm(*a, *b), *out);
    }
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
    assert_eq!(f + f2, Fraction::from_string("3/5").unwrap());
}

#[test]
fn test_adding_different_signs() {
    let f = Fraction::from_string("-1/5").unwrap();
    let f2 = Fraction::from_string("2/5").unwrap();
    assert_eq!(f + f2, Fraction::from_string("1/5").unwrap());

    let f = Fraction::from_string("1/5").unwrap();
    let f2 = Fraction::from_string("-2/5").unwrap();
    assert_eq!(f + f2, Fraction::from_string("-1/5").unwrap());
}

#[test]
fn test_adding_reduces() {
    let f = Fraction::from_string("1/3").unwrap();
    let f2 = Fraction::from_string("2/3").unwrap();
    assert!((f + f2).debug_eq(&Fraction::from_string("1").unwrap()));
}

#[test]
fn test_normal_subtraction() {
    let f = Fraction::from_string("6/5").unwrap();
    let f2 = Fraction::from_string("4/5").unwrap();
    assert_eq!(f - f2, Fraction::from_string("2/5").unwrap());
}

#[test]
fn test_different_sign_subtraction() {
    let f = Fraction::from_string("6/5").unwrap();
    let f2 = Fraction::from_string("-4/5").unwrap();
    assert_eq!(f - f2, Fraction::from_string("2").unwrap());

    let f = Fraction::from_string("-6/5").unwrap();
    let f2 = Fraction::from_string("4/5").unwrap();
    assert_eq!(f - f2, Fraction::from_string("-2").unwrap());
}

#[test]
fn test_comparison() {
    assert!(Fraction::new(1, 5) == Fraction::new(2, 10));
    assert!(Fraction::new(1, 5) == Fraction::new(1, 5));
    assert!(Fraction::new(2, 5) != Fraction::new(1, 5));

    assert!(Fraction::new(2, 5) > Fraction::new(2, 10));
    assert!(Fraction::new(1, 10) < Fraction::new(1, 5));
}

#[test]
fn test_multiply() {
    assert_eq!(
        Fraction::new(5, 3) * Fraction::new(2, 5),
        Fraction::new(10, 15)
    );
    assert_eq!(
        Fraction::new(5, 3) * Fraction::new(2, 5),
        Fraction::new(2, 3)
    );
}

#[test]
fn test_divide() {
    assert_eq!(
        Fraction::new(5, 3) / Fraction::new(5, 2),
        Fraction::new(10, 15)
    );
}
