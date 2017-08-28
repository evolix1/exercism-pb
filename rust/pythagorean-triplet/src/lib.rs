// I like brute force :)

pub fn find() -> Option<u32> {
    find_for(1000)
}

#[derive(Clone)]
struct Candidate {
    a: u32,
    b: u32,
    c: u32
}

struct Combinations {
    max: u32,
    last: Option<Candidate>
}

impl Candidate {
    fn sum(&self) -> u32 {
        self.a + self.b + self.c
    }

    fn product(&self) -> u32 {
        self.a * self.b * self.c
    }

    fn is_triplet(&self) -> bool {
        (self.a * self.a) + (self.b * self.b) == (self.c * self.c)
    }
}

impl Combinations {
    fn new(sum_max: u32) -> Combinations {
        Combinations {
            max: sum_max,
            last: Some(Candidate {
                a: 1, b: 2, c: 3
            })
        }
    }

    fn next_candidate(&self) -> Option<Candidate> {
        self.last.clone().and_then(|x| {
            self.next_candidate_c(&x)
                .or_else(|| self.next_candidate_b(&x))
                .or_else(|| self.next_candidate_a(&x))
        })
    }

    fn filter(&self, next: Candidate) -> Option<Candidate> {
        if next.sum() <= self.max { Some(next) }
        else { None }
    }

    fn next_candidate_c(&self, last: &Candidate) -> Option<Candidate> {
        self.filter(Candidate { a: last.a, b: last.b, c: last.c + 1 })
    }

    fn next_candidate_b(&self, last: &Candidate) -> Option<Candidate> {
        self.filter(Candidate { a: last.a, b: last.b + 1, c: last.b + 2 })
    }

    fn next_candidate_a(&self, last: &Candidate) -> Option<Candidate> {
        self.filter(Candidate { a: last.a + 1, b: last.a + 2, c: last.a + 3 })
    }
}

impl Iterator for Combinations {
    type Item = Candidate;

    fn next(&mut self) -> Option<Candidate> {
        self.last = self.next_candidate();
        self.last.clone()
    }
}

pub fn find_for(sum: u32) -> Option<u32> {
    Combinations::new(sum)
        .filter(|ref x| x.is_triplet())
        .filter(|ref x| x.sum() == sum)
        .next()
        .map(|x| x.product())
}
