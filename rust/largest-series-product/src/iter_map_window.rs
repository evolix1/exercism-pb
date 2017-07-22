use std::cmp;

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct MapWindow<I, F, W> {
    winsize: usize,
    iter: I,
    mutate: F,
    win: Vec<W> // FIFO
}

impl<I, F, W, R> Iterator for MapWindow<I, F, W> where
        I: Iterator<Item=W>,
        F: FnMut(&[W]) -> R {
    type Item = R;

    fn next(&mut self) -> Option<R> {
        let first_run = self.win.is_empty();
        if first_run {
            // fill up the window
            for _ in 0..self.winsize-1 {
                self.win.push(self.iter
                              .next()
                              .expect("item must exist, see size_hint"));
            }
        } else {
            // discard oldest element of the window
            self.win.remove(0);
        }

        match self.iter.next() {
            Some(item) => {
                self.win.push(item);
                Some((self.mutate)(self.win.as_slice()))
            },
            None => None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        let never_run = self.win.is_empty();

        match never_run {
            true => {
                let lower = cmp::max(0, lower - self.winsize + 1);
                let upper = match upper {
                    Some(x) if x >= self.winsize => Some(x - self.winsize + 1),
                    _ => None
                };
                (lower, upper)
            },
            false => (lower, upper)
        }


    }
}

pub trait IterMapWindow : Iterator {
    fn map_win<F, R>(self, size: usize, func: F) -> MapWindow<Self, F, Self::Item> where
        Self: Sized,
        F: FnMut(&[Self::Item]) -> R {
            assert!(size != 0, "size must be positive, not null");
            MapWindow {
                winsize: size,
                iter: self,
                mutate: func,
                win: Vec::new()
            }
        }
}

impl<I: Iterator> IterMapWindow for I {}
