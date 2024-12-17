use std::str::FromStr;

pub trait Parse {
    fn after_colon(&self) -> &str;

    fn parse_after_colon<T>(&self) -> T
    where
        T: FromStr,
        T::Err: std::fmt::Debug;

    fn list<T>(&self, separator: &str) -> Vec<T>
    where
        T: FromStr,
        T::Err: std::fmt::Debug;

    fn pair<T>(&self, separator: &str) -> (T, T)
    where
        T: FromStr,
        T::Err: std::fmt::Debug;

    fn parse_regex<T, U, F, const N: usize>(&self, regex: &str, f: F) -> Vec<T>
    where
        F: Fn([U; N]) -> T,
        U: FromStr,
        U::Err: std::fmt::Debug;
}

impl Parse for str {
    fn after_colon(&self) -> &str {
        self.split_once(": ").unwrap().1
    }

    fn parse_after_colon<T>(&self) -> T
    where
        T: FromStr,
        T::Err: std::fmt::Debug,
    {
        self.after_colon().parse().unwrap()
    }

    fn list<T>(&self, separator: &str) -> Vec<T>
    where
        T: FromStr,
        T::Err: std::fmt::Debug,
    {
        self.split(separator).map(|n| n.parse().unwrap()).collect()
    }

    fn pair<T>(&self, separator: &str) -> (T, T)
    where
        T: FromStr,
        T::Err: std::fmt::Debug,
    {
        let (a, b) = self.split_once(separator).unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }

    fn parse_regex<T, U, F, const N: usize>(&self, regex: &str, f: F) -> Vec<T>
    where
        F: Fn([U; N]) -> T,
        U: FromStr,
        U::Err: std::fmt::Debug,
    {
        let re = regex::Regex::new(regex).unwrap();
        re.captures_iter(self)
            .map(|c| {
                let (_, values) = c.extract();
                let parsed = values.map(|v| v.parse().unwrap());
                f(parsed)
            })
            .collect()
    }
}
