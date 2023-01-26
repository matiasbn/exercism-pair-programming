#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    config: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    config: String,
}

const VALID_DNA: &[char] = &['G', 'C', 'T', 'A'];
const VALID_RNA: &[char] = &['C', 'G', 'A', 'U'];

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna.chars().find(|l| !VALID_DNA.contains(l)) {
            Some(invalid) => {
                let invalid_index = dna.chars().position(|c| c == invalid).unwrap();
                Err(invalid_index)
            }
            None => Ok(Dna {
                config: dna.to_string(),
            }),
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            config: String::from_iter(self.config.chars().map(|c| match c {
                'G' => "C",
                'C' => "G",
                'T' => "A",
                'A' => "U",
                _ => todo!(),
            })),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna.chars().find(|l| !VALID_RNA.contains(l)) {
            Some(invalid) => {
                let invalid_index = rna.chars().position(|l| l == invalid).unwrap();
                Err(invalid_index)
            }
            None => Ok(Rna {
                config: rna.to_string(),
            }),
        }
    }
}
