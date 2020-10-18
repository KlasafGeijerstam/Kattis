use std::io::prelude::*;
use std::str::{from_utf8_unchecked, FromStr};

pub struct FastInput {
    data: Vec<u8>,
    pos: usize,
    emptied: bool,
}

#[allow(dead_code)]
impl FastInput {
    pub fn new() -> Self {
        FastInput {
            data: FastInput::read_to_end(8196),
            pos: 0,
            emptied: false,
        }
    }

    fn read_to_end(buffer_size: usize) -> Vec<u8> {
        let mut data = Vec::with_capacity(buffer_size);
        std::io::stdin().lock().read_to_end(&mut data).unwrap();
        data
    }

    pub fn with_buffer_size(buffer_size: usize) -> Self {
        FastInput {
            data: FastInput::read_to_end(buffer_size),
            pos: 0,
            emptied: false,
        }
    }

    fn next_newline(&mut self) -> Option<usize> {
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

    pub fn next<T: FromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let mut it = self.next_as_iter();
        it.next().unwrap()
    }

    pub fn next_tuple<T1: FromStr, T2: FromStr>(&mut self) -> (T1, T2)
    where
        <T1 as FromStr>::Err: std::fmt::Debug,
        <T2 as FromStr>::Err: std::fmt::Debug,
    {
        let mut it = self.next_split();
        (
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        )
    }

    pub fn next_triple<T1: FromStr, T2: FromStr, T3: FromStr>(&mut self) -> (T1, T2, T3)
    where
        <T1 as FromStr>::Err: std::fmt::Debug,
        <T2 as FromStr>::Err: std::fmt::Debug,
        <T3 as FromStr>::Err: std::fmt::Debug,
    {
        let mut it = self.next_split();
        (
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        )
    }

    pub fn next_quad<T1: FromStr, T2: FromStr, T3: FromStr, T4: FromStr>(
        &mut self,
    ) -> (T1, T2, T3, T4)
    where
        <T1 as FromStr>::Err: std::fmt::Debug,
        <T2 as FromStr>::Err: std::fmt::Debug,
        <T3 as FromStr>::Err: std::fmt::Debug,
        <T4 as FromStr>::Err: std::fmt::Debug,
    {
        let mut it = self.next_split();
        (
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        )
    }

    pub fn next_quintuple<T1: FromStr, T2: FromStr, T3: FromStr, T4: FromStr, T5: FromStr>(
        &mut self,
    ) -> (T1, T2, T3, T4, T5)
    where
        <T1 as FromStr>::Err: std::fmt::Debug,
        <T2 as FromStr>::Err: std::fmt::Debug,
        <T3 as FromStr>::Err: std::fmt::Debug,
        <T4 as FromStr>::Err: std::fmt::Debug,
        <T5 as FromStr>::Err: std::fmt::Debug,
    {
        let mut it = self.next_split();
        (
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        )
    }

    pub fn next_as_iter<T: FromStr>(&mut self) -> impl Iterator<Item = T> + '_
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.next_line()
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
    }

    pub fn next_split(&mut self) -> impl Iterator<Item = &str> + '_ {
        self.next_line().trim().split(' ')
    }

    pub fn has_next_line(&self) -> bool {
        self.pos != self.data.len()
    }

    pub fn next_str_tuple(&mut self) -> (&str, &str) {
        let mut line = self.next_line().trim().split(' ');
        (line.next().unwrap(), line.next().unwrap())
    }

    pub fn next_str_triple(&mut self) -> (&str, &str, &str) {
        let mut line = self.next_line().trim().split(' ');
        (
            line.next().unwrap(),
            line.next().unwrap(),
            line.next().unwrap(),
        )
    }
}
