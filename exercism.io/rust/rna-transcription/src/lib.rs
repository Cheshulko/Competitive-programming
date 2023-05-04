#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    dna: String,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Rna {
    rna: String,
}

impl Dna {
    const DNA_VALID: [char; 4] = ['G', 'C', 'T', 'A'];

    pub fn new<T: AsRef<str>>(dna: T) -> Result<Dna, usize> {
        if let Some(invalid) = dna
            .as_ref()
            .char_indices()
            .find(|(_, c)| !Dna::DNA_VALID.contains(&c))
            .map(|(ind, _)| ind)
        {
            Err(invalid)
        } else {
            Ok(Dna {
                dna: dna.as_ref().to_string(),
            })
        }
    }

    pub fn into_rna(self) -> Rna {
        let m = Dna::DNA_VALID
            .iter()
            .zip(Rna::RNA_VALID.iter())
            .collect::<Vec<(&char, &char)>>();

        self.dna
            .chars()
            .map(|c| *m.iter().find(|x| x.0 == &c).unwrap().1)
            .collect::<Rna>()
    }
}

impl Rna {
    const RNA_VALID: [char; 4] = ['C', 'G', 'A', 'U'];

    pub fn add(&mut self, c: char) {
        self.rna.push(c);
    }

    pub fn new<T: AsRef<str>>(rna: T) -> Result<Rna, usize> {
        if let Some(invalid) = rna
            .as_ref()
            .char_indices()
            .find(|(_, c)| !Rna::RNA_VALID.contains(&c))
            .map(|(ind, _)| ind)
        {
            Err(invalid)
        } else {
            Ok(Rna {
                rna: rna.as_ref().to_string(),
            })
        }
    }
}

impl FromIterator<char> for Rna {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = char>,
    {
        let mut rna = Rna::default();
        iter.into_iter().for_each(|ch| rna.add(ch));
        rna
    }
}
