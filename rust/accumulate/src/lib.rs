
// I'm too lazy to do implement lazy

use std::iter::IntoIterator;

pub fn map_function<T, I, F, V>(data: T, func: &F) -> Vec<V>
where
    T: IntoIterator<Item = I>,
    F: Fn(I) -> V,
{
    let mut res = Vec::new();
    for d in data {
        res.push(func(d));
    }
    res
}

pub fn map_closure<T, I, F, V>(data: T, func: F) -> Vec<V>
where
    T: IntoIterator<Item = I>,
    F: Fn(I) -> V,
{
    let mut res = Vec::new();
    for d in data {
        res.push(func(d));
    }
    res
}

