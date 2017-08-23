
pub struct Allergies(u32);

impl Allergies {
    pub fn new(x: u32) -> Allergies {
        Allergies(x)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        allergen.value() & self.0 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::all()
            .into_iter()
            .filter(|ref a| self.is_allergic_to(&a))
            .collect()
    }
}

macro_rules! allergens {
    { $( $name:ident => $order:expr ),* }
    => {
        #[derive(Debug, PartialEq)]
        pub enum Allergen {
            $( $name ),*
        }

        impl Allergen {
            fn value(&self) -> u32 {
                match *self {
                    $( Allergen::$name => $order ),*
                }
            }

            fn all() -> Vec<Allergen> {
                vec![ $( Allergen::$name ),* ]
            }
        }

    }
}

allergens!{
    Eggs         => 1,
    Peanuts      => 2,
    Shellfish    => 4,
    Strawberries => 8,
    Tomatoes     => 16,
    Chocolate    => 32,
    Pollen       => 64,
    Cats         => 128
}
