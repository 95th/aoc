use std::str::FromStr;

pub trait Parse {
    fn after_colon(&self) -> &str;

    fn parse_after_colon<T>(&self) -> T
    where
        T: FromStr;

    fn list<T>(&self, separator: &str) -> Vec<T>
    where
        T: FromStr;

    fn pair<T>(&self, separator: &str) -> (T, T)
    where
        T: FromStr;

    fn parse_regex<T, U, F, const N: usize>(&self, regex: &str, f: F) -> Vec<T>
    where
        F: Fn([U; N]) -> T,
        U: FromStr;
}

impl Parse for str {
    fn after_colon(&self) -> &str {
        self.split_once(": ").unwrap().1
    }

    fn parse_after_colon<T>(&self) -> T
    where
        T: FromStr,
    {
        self.after_colon().parse().ok().unwrap()
    }

    fn list<T>(&self, separator: &str) -> Vec<T>
    where
        T: FromStr,
    {
        self.split(separator)
            .map(|n| n.parse().ok().unwrap())
            .collect()
    }

    fn pair<T>(&self, separator: &str) -> (T, T)
    where
        T: FromStr,
    {
        let (a, b) = self.split_once(separator).unwrap();
        (a.parse().ok().unwrap(), b.parse().ok().unwrap())
    }

    fn parse_regex<T, U, F, const N: usize>(&self, regex: &str, f: F) -> Vec<T>
    where
        F: Fn([U; N]) -> T,
        U: FromStr,
    {
        let re = regex::Regex::new(regex).unwrap();
        re.captures_iter(self)
            .map(|c| {
                let (_, values) = c.extract();
                let parsed = values.map(|v| v.parse().ok().unwrap());
                f(parsed)
            })
            .collect()
    }
}
