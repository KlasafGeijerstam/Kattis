use std::io::prelude::*;
use std::io::BufReader;

pub struct Inp {
    buffer: String,
    reader: BufReader<std::io::Stdin>,
}

#[allow(dead_code)]
impl Inp {
    pub fn new() -> Self {
        Inp {
            buffer: String::new(),
            reader: BufReader::new(std::io::stdin()),
        }
    }

    pub fn next_line(&mut self) -> &str {
        self.buffer.clear();
        self.reader.read_line(&mut self.buffer).unwrap();

        &self.buffer
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let mut it = self.next_as_iter();
        it.next().unwrap()
    }

    pub fn next_tuple<T: std::str::FromStr>(&mut self) -> (T, T)
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let mut it = self.next_as_iter();
        (it.next().unwrap(), it.next().unwrap())
    }

    pub fn next_triple<T: std::str::FromStr>(&mut self) -> (T, T, T)
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let mut it = self.next_as_iter();
        (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
    }

    pub fn next_quad<T: std::str::FromStr>(&mut self) -> (T, T, T, T)
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let mut it = self.next_as_iter();
        (
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
        )
    }

    pub fn next_quintuple<T: std::str::FromStr>(&mut self) -> (T, T, T, T, T)
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let mut it = self.next_as_iter();
        (
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
        )
    }

    pub fn next_as_iter<'a, T: std::str::FromStr>(&'a mut self) -> Box<dyn Iterator<Item = T> + 'a>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        Box::new(
            self.next_line()
                .trim()
                .split(' ')
                .map(|x| x.parse().unwrap()),
        )
    }

    pub fn get_next_line(&mut self) -> Option<&str> {
        self.buffer.clear();
        match self.reader.read_line(&mut self.buffer) {
            Ok(n) if n > 0 => Some(&self.buffer),
            _ => None,
        }
    }

    pub fn get_next<T: std::str::FromStr>(&mut self) -> Option<T>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        if let Some(mut it) = self.get_next_as_iter() {
            Some(it.next().unwrap())
        } else {
            None
        }
    }

    pub fn get_next_tuple<T: std::str::FromStr>(&mut self) -> Option<(T, T)>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        if let Some(mut it) = self.get_next_as_iter() {
            Some((it.next().unwrap(), it.next().unwrap()))
        } else {
            None
        }
    }

    pub fn get_next_triple<T: std::str::FromStr>(&mut self) -> Option<(T, T, T)>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        if let Some(mut it) = self.get_next_as_iter() {
            Some((it.next().unwrap(), it.next().unwrap(), it.next().unwrap()))
        } else {
            None
        }
    }

    pub fn get_next_quad<T: std::str::FromStr>(&mut self) -> Option<(T, T, T, T)>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        if let Some(mut it) = self.get_next_as_iter() {
            Some((
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
            ))
        } else {
            None
        }
    }

    pub fn get_next_quintuple<T: std::str::FromStr>(&mut self) -> Option<(T, T, T, T, T)>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        if let Some(mut it) = self.get_next_as_iter() {
            Some((
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
            ))
        } else {
            None
        }
    }

    pub fn get_next_as_iter<'a, T: std::str::FromStr>(
        &'a mut self,
    ) -> Option<Box<dyn Iterator<Item = T> + 'a>>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        if let Some(l) = self.get_next_line() {
            Some(Box::new(l.trim().split(' ').map(|x| x.parse().unwrap())))
        } else {
            None
        }
    }

    pub fn next_str_tuple(&mut self) -> (&str, &str) {
        let mut line = self.next_line().trim().split(' ');
        (line.next().unwrap(), line.next().unwrap())
    }
}
