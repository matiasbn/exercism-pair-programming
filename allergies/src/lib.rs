pub struct Allergies {
    vector_of_allergies: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats, //128
    Work, // 256
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut vector = vec![1, 2, 4, 8, 16, 32, 64, 128];
        vector.reverse();
        let mut allergies: Vec<Allergen> = vec![];
        let mut score_result = score % 256;
        for (idx, value) in vector.into_iter().enumerate() {
            if score_result >= value {
                match idx {
                    0 => allergies.push(Allergen::Cats),
                    1 => allergies.push(Allergen::Pollen),
                    2 => allergies.push(Allergen::Chocolate),
                    3 => allergies.push(Allergen::Tomatoes),
                    4 => allergies.push(Allergen::Strawberries),
                    5 => allergies.push(Allergen::Shellfish),
                    6 => allergies.push(Allergen::Peanuts),
                    7 => allergies.push(Allergen::Eggs),
                    8 => allergies.push(Allergen::Work),
                    _ => panic!("bad, LOL"),
                }
                score_result -= value;
            }
        }

        Allergies {
            vector_of_allergies: allergies,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.vector_of_allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let vec = self.vector_of_allergies.clone();
        vec
    }
}
