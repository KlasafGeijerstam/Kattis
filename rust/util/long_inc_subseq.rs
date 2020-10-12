fn ceil_index<T: PartialOrd>(v: &[T], mut l: i64, mut r: i64, key: &T) -> usize {
    while r - l > 1 {
        let m = l + (r - l) / 2;
        if &v[m as usize] >= key {
            r = m;
        } else {
            l = m
        }
    }
    r as usize
}

pub fn longest_increasing_subsequence<T>(v: &[T]) -> usize
where
    T: PartialOrd + Default + Clone + Copy,
{
    if v.is_empty() {
        return 0;
    }
    let mut tail = vec![T::default(); v.len()];
    let mut length = 1;

    tail[0] = v[0];
    for i in 1..v.len() {
        if v[i] < tail[0] {
            tail[0] = v[i];
        } else if v[i] > tail[length - 1] {
            tail[length] = v[i];
            length += 1;
        } else {
            let index = ceil_index(&tail, -1, length as i64 - 1, &v[i]);
            tail[index] = v[i];
        }
    }
    length 
}
