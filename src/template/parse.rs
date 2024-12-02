use std::convert::Infallible;
use std::fmt::Debug;
use std::str::FromStr;

/// Similar to `std::str::FromStr`, but supports borrowing (e.g. `&str`), and requires `Err: Debug`.
pub trait MyFromStr<'a>: Sized {
    type Err: Debug;

    fn from_str(s: &'a str) -> Result<Self, Self::Err>;
}

macro_rules! impl_my_from_str {
    ($($t:ty)*) => {
        $(impl<'a> MyFromStr<'a> for $t {
            type Err = <$t as FromStr>::Err;

            fn from_str(s: &'a str) -> Result<Self, Self::Err> {
                <$t as FromStr>::from_str(s)
            }
        })*
    };
}

impl_my_from_str!(char f32 f64 i8 u8 i16 u16 i32 u32 i64 u64 isize usize String);

impl<'a> MyFromStr<'a> for &'a str {
    type Err = Infallible;

    fn from_str(s: &'a str) -> Result<Self, Self::Err> {
        Ok(s)
    }
}

/// Trait to add a `my_parse()` method to `str`
pub trait MyParse {
    fn my_parse<'a, F: MyFromStr<'a>>(&'a self) -> Result<F, F::Err>;
}

impl MyParse for str {
    fn my_parse<'a, F: MyFromStr<'a>>(&'a self) -> Result<F, F::Err> {
        F::from_str(self)
    }
}

pub fn lines(input: &str) -> impl Iterator<Item = &str> {
    input.split('\n')
}

#[cold]
fn cold() {}

/// Parse one item from each line
pub fn split1<'a, T: MyFromStr<'a>>(input: &'a str) -> impl 'a + Iterator<Item = T> {
    lines(input).filter_map(|line| {
        let line = line.trim_ascii();
        if line.is_empty() {
            // skip blank lines
            cold();
            None
        } else {
            Some(line.my_parse().unwrap())
        }
    })
}

/// Parse 2 items from each line, separated by ASCII whitespace
pub fn split2<'a, T0: MyFromStr<'a>, T1: MyFromStr<'a>>(
    input: &'a str,
) -> impl 'a + Iterator<Item = (T0, T1)> {
    lines(input).filter_map(|line| {
        let mut parts = line.split_ascii_whitespace();
        let Some(part0) = parts.next() else {
            // skip blank lines
            cold();
            return None;
        };
        let val0 = part0.my_parse().unwrap();
        let val1 = parts.next().unwrap().my_parse().unwrap();
        assert!(parts.next().is_none(), "found extra data on line");
        Some((val0, val1))
    })
}

/// Parse 3 items from each line, separated by ASCII whitespace
pub fn split3<'a, T0: MyFromStr<'a>, T1: MyFromStr<'a>, T2: MyFromStr<'a>>(
    input: &'a str,
) -> impl 'a + Iterator<Item = (T0, T1, T2)> {
    lines(input).filter_map(|line| {
        let mut parts = line.split_ascii_whitespace();
        let Some(part0) = parts.next() else {
            // skip blank lines
            cold();
            return None;
        };
        let val0 = part0.my_parse().unwrap();
        let val1 = parts.next().unwrap().my_parse().unwrap();
        let val2 = parts.next().unwrap().my_parse().unwrap();
        assert!(parts.next().is_none(), "found extra data on line");
        Some((val0, val1, val2))
    })
}

/// Parse 4 items from each line, separated by ASCII whitespace
pub fn split4<'a, T0: MyFromStr<'a>, T1: MyFromStr<'a>, T2: MyFromStr<'a>, T3: MyFromStr<'a>>(
    input: &'a str,
) -> impl 'a + Iterator<Item = (T0, T1, T2, T3)> {
    lines(input).filter_map(|line| {
        let mut parts = line.split_ascii_whitespace();
        let Some(part0) = parts.next() else {
            // skip blank lines
            cold();
            return None;
        };
        let val0 = part0.my_parse().unwrap();
        let val1 = parts.next().unwrap().my_parse().unwrap();
        let val2 = parts.next().unwrap().my_parse().unwrap();
        let val3 = parts.next().unwrap().my_parse().unwrap();
        assert!(parts.next().is_none(), "found extra data on line");
        Some((val0, val1, val2, val3))
    })
}

/// Parse 5 items from each line, separated by ASCII whitespace
pub fn split5<
    'a,
    T0: MyFromStr<'a>,
    T1: MyFromStr<'a>,
    T2: MyFromStr<'a>,
    T3: MyFromStr<'a>,
    T4: MyFromStr<'a>,
>(
    input: &'a str,
) -> impl 'a + Iterator<Item = (T0, T1, T2, T3, T4)> {
    lines(input).filter_map(|line| {
        let mut parts = line.split_ascii_whitespace();
        let Some(part0) = parts.next() else {
            // skip blank lines
            cold();
            return None;
        };
        let val0 = part0.my_parse().unwrap();
        let val1 = parts.next().unwrap().my_parse().unwrap();
        let val2 = parts.next().unwrap().my_parse().unwrap();
        let val3 = parts.next().unwrap().my_parse().unwrap();
        let val4 = parts.next().unwrap().my_parse().unwrap();
        assert!(parts.next().is_none(), "found extra data on line");
        Some((val0, val1, val2, val3, val4))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_strs() {
        let input = "
foo  \t bar
\t
123 ugh
        ";

        let result = split2::<&str, &str>(input).collect::<Vec<_>>();
        assert_eq!(result, &[("foo", "bar"), ("123", "ugh")]);
    }
}
