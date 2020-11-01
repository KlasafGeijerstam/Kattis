use std::cell::Cell;
use std::fmt::Display;
use std::io::prelude::*;
use std::io::stdin;
use std::ops::Deref;
use std::str::{from_utf8_unchecked, FromStr};

#[cfg(test)]
mod tests;

pub struct FastInput {
    data: Vec<u8>,
    pos: Cell<usize>,
}

const BUFFER_SIZE: usize = 8196;

#[allow(dead_code)]
impl FastInput {
    pub fn new() -> Self {
        FastInput {
            data: FastInput::read_to_end(stdin().lock(), BUFFER_SIZE),
            pos: Cell::new(0),
        }
    }

    pub fn with_buffer_size(buffer_size: usize) -> Self {
        FastInput {
            data: FastInput::read_to_end(stdin().lock(), buffer_size),
            pos: Cell::new(0),
        }
    }

    pub fn with_reader<T: Read>(input: T) -> Self {
        FastInput {
            data: FastInput::read_to_end(input, BUFFER_SIZE),
            pos: Cell::new(0),
        }
    }

    pub fn next_line(&self) -> &str {
        if let Some(nline) = self.next_newline() {
            unsafe {
                let pos = self.pos.get();
                let s = from_utf8_unchecked(&self.data[pos..nline]);
                self.pos.set(nline + 1);
                s
            }
        } else {
            unsafe {
                let s = from_utf8_unchecked(&self.data[self.pos.get()..]);
                self.pos.set(self.data.len());
                s
            }
        }
    }

    pub fn next<'a, T: FastParse<'a>>(&'a self) -> T {
        let mut it = self.next_as_iter();
        it.next().unwrap()
    }

    pub fn next_tuple<'a, T1: FastParse<'a>, T2: FastParse<'a>>(&'a self) -> (T1, T2) {
        let mut it = self.next_split();
        (
            T1::fparse(it.next().unwrap()),
            T2::fparse(it.next().unwrap()),
        )
    }

    pub fn next_triple<'a, T1: FastParse<'a>, T2: FastParse<'a>, T3: FastParse<'a>>(
        &'a self,
    ) -> (T1, T2, T3) {
        let mut it = self.next_split();
        (
            T1::fparse(it.next().unwrap()),
            T2::fparse(it.next().unwrap()),
            T3::fparse(it.next().unwrap()),
        )
    }

    pub fn next_quad<
        'a,
        T1: FastParse<'a>,
        T2: FastParse<'a>,
        T3: FastParse<'a>,
        T4: FastParse<'a>,
    >(
        &'a self,
    ) -> (T1, T2, T3, T4) {
        let mut it = self.next_split();
        (
            T1::fparse(it.next().unwrap()),
            T2::fparse(it.next().unwrap()),
            T3::fparse(it.next().unwrap()),
            T4::fparse(it.next().unwrap()),
        )
    }

    pub fn next_quintuple<
        'a,
        T1: FastParse<'a>,
        T2: FastParse<'a>,
        T3: FastParse<'a>,
        T4: FastParse<'a>,
        T5: FastParse<'a>,
    >(
        &'a self,
    ) -> (T1, T2, T3, T4, T5) {
        let mut it = self.next_split();
        (
            T1::fparse(it.next().unwrap()),
            T2::fparse(it.next().unwrap()),
            T3::fparse(it.next().unwrap()),
            T4::fparse(it.next().unwrap()),
            T5::fparse(it.next().unwrap()),
        )
    }

    pub fn next_as_iter<'a, T: FastParse<'a>>(&'a self) -> impl Iterator<Item = T> + '_ {
        self.next_line().trim().split(' ').map(|x| T::fparse(x))
    }

    pub fn next_split<'a>(&'a self) -> impl Iterator<Item = &'a str> + '_ {
        self.next_line().trim().split(' ')
    }

    pub fn has_next_line(&self) -> bool {
        self.pos.get() != self.data.len()
    }

    #[deprecated(
        since = "0.1.1",
        note = "Use `next_tuple` with the `Str` type instead."
    )]
    pub fn next_str_tuple(&self) -> (&str, &str) {
        let mut line = self.next_line().trim().split(' ');
        (line.next().unwrap(), line.next().unwrap())
    }

    #[deprecated(
        since = "0.1.1",
        note = "Use `next_triple` with the `Str` type instead."
    )]
    pub fn next_str_triple(&self) -> (&str, &str, &str) {
        let mut line = self.next_line().trim().split(' ');
        (
            line.next().unwrap(),
            line.next().unwrap(),
            line.next().unwrap(),
        )
    }

    fn read_to_end<T: Read>(mut input: T, buffer_size: usize) -> Vec<u8> {
        let mut data = Vec::with_capacity(buffer_size);
        input.read_to_end(&mut data).unwrap();
        data
    }

    fn next_newline(&self) -> Option<usize> {
        let mut i = self.pos.get();
        while i < self.data.len() && self.data[i] != b'\n' {
            i += 1;
        }
        if i < self.data.len() && self.data[i] == b'\n' {
            Some(i)
        } else {
            None
        }
    }
}

impl Default for FastInput {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FastParse<'a> {
    fn fparse(s: &'a str) -> Self;
}

impl<'a, T: FromStr> FastParse<'a> for T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    fn fparse(s: &'a str) -> Self {
        s.parse().unwrap()
    }
}

pub struct Str<'a>(&'a str);

impl<'a> FastParse<'a> for Str<'a> {
    fn fparse(s: &'a str) -> Self {
        Str::<'a>(s)
    }
}

impl<'a> Deref for Str<'a> {
    type Target = &'a str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Str<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(fmt)
    }
}
