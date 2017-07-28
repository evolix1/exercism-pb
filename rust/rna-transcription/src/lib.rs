
impl<'a> DeoxyribonucleicAcid<'a> {
    pub fn to_rna<'b>(&self) -> Result<RibonucleicAcid<'b>, ()> {
        let res = self.0
            .chars()
            .into_iter()
            .filter_map(|x| match x {
                'G' => Some('C'),
                'C' => Some('G'),
                'T' => Some('A'),
                'A' => Some('U'),
                _ => None,
            })
            .fuse()
            .collect::<String>();

        // if we got an illegal char, result is shorter than input
        match res.len() == self.0.len() {
            true => Ok(RibonucleicAcid(Cow::Owned(res))),
            false => Err(()),
        }
    }
}


// Boring stuff bellow this point (struct, ctor...)

use std::borrow::Cow;

#[derive(Debug)]
pub struct RibonucleicAcid<'a>(Cow<'a, str>);

#[derive(Debug)]
pub struct DeoxyribonucleicAcid<'a>(Cow<'a, str>);

impl<'a> RibonucleicAcid<'a> {
    pub fn new<T>(s: T) -> RibonucleicAcid<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        RibonucleicAcid(s.into())
    }
}

impl<'a> DeoxyribonucleicAcid<'a> {
    pub fn new(s: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid(Cow::Borrowed(s))
    }
}

impl<'a> PartialEq for RibonucleicAcid<'a> {
    fn eq(&self, other: &RibonucleicAcid) -> bool {
        self.0 == other.0
    }
}
