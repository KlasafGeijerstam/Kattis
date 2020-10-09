use std::io::prelude::*;
use std::str::from_utf8_unchecked;

pub struct Inp {
    data: Vec<u8>,
    pos: usize,
    emptied: bool,
}

#[allow(dead_code)]
impl Inp {
    pub fn new() -> Self {
        let mut data = Vec::with_capacity(8196);
        std::io::stdin().lock().read_to_end(&mut data).unwrap();
        Inp {
            data,
            pos: 0,
            emptied: false,
        }
    }

    fn next_newline(&self) -> Option<usize> {
        let mut i = self.pos;
        while i < self.data.len() && self.data[i] != '\n' as u8 {
            i += 1;
        }
        if i < self.data.len() && self.data[i] == '\n' as u8 {
            Some(i)
        } else {
            None
        }
    }

    pub fn next_line(&mut self) -> &str {
        if let Some(nline) = self.next_newline() {
            unsafe {
                let s = from_utf8_unchecked(&self.data[self.pos..nline]);
                self.pos = nline + 1;
                s
            }
        } else {
            unsafe {
                self.emptied = true;
                from_utf8_unchecked(&self.data[self.pos..])
            }
        }
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

    pub fn next_as_iter<T: std::str::FromStr>(&mut self) -> impl Iterator<Item = T> + '_
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.next_line()
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
    }

    pub fn get_next_line(&mut self) -> Option<&str> {
        if self.emptied {
            return None;
        }
        Some(self.next_line())
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

    pub fn get_next_as_iter<T: std::str::FromStr>(&mut self) -> Option<impl Iterator<Item = T> + '_>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        if let Some(l) = self.get_next_line() {
            if l.len() == 0 {
                return None
            }
            Some(l.trim().split(' ').map(|x| x.parse().unwrap()))
        } else {
            None
        }
    }

    pub fn next_str_tuple(&mut self) -> (&str, &str) {
        let mut line = self.next_line().trim().split(' ');
        (line.next().unwrap(), line.next().unwrap())
    }
}
