pub struct Allergies {
    pub score: u32,
    binary_repr: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        //todo!("Given the '{score}' score, construct a new Allergies struct.");
        let binary_repr = Self::to_binary(score);
        Allergies { score, binary_repr }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        //todo!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
        if self.binary_repr.is_empty() {
                    return false;
                }
        match allergen {
            Allergen::Eggs => self.binary_repr[0] == 1,
            Allergen::Peanuts=> {
                if self.binary_repr.len() < 2 {
                    return false;
                }
                self.binary_repr[1] == 1
            },
            Allergen::Shellfish=>{
                if self.binary_repr.len() < 3 {
                    return false;
                }
                self.binary_repr[2] == 1
            },
            Allergen::Strawberries=>{
                if self.binary_repr.len() < 4 {
                    return false;
                }
                self.binary_repr[3] == 1
            },
            Allergen::Tomatoes=>{
                if self.binary_repr.len() < 5 {
                    return false;
                }
                self.binary_repr[4] == 1
            },
            Allergen::Chocolate=>{
                if self.binary_repr.len() < 6 {
                    return false;
                }
                self.binary_repr[5] == 1
            },
            Allergen::Pollen=>{
                if self.binary_repr.len() < 7 {
                    return false;
                }
                self.binary_repr[6] == 1
            },
            Allergen::Cats=>{
                if self.binary_repr.len() < 8 {
                    return false;
                }
                self.binary_repr[7] == 1
            },
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        use Allergen::*;
        //todo!(
            //"Return the list of allergens contained within the score with which the Allergies struct was made."
        //);
        let mut result = vec![];
        let end = if self.binary_repr.len() < 9 {
            self.binary_repr.len()
        }else{   
            9
        };
        let allergens = [Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats,];
        for allergen in allergens {
            if self.is_allergic_to(&allergen) {
                result.push(allergen);
            }
        }

        result
    }
    
    fn to_binary(score: u32)->Vec<u8> {
        let mut result = vec![];
        let mut score = score;
        while score > 0 {
            result.push((score%2) as u8);
            score /= 2;
        }
        result
    }
}

