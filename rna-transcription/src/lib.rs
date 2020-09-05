#[derive(Debug, PartialEq)]
pub struct DNA {
    pub dna: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    pub rna: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        for (i, c) in dna.char_indices() {
            if !['A', 'C', 'G', 'T'].contains(&c) {
                return Err(i);
            }
        }
        Ok(DNA {
            dna: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> RNA {
        let rna = self
            .dna
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect::<String>();
        RNA { rna }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (i, c) in rna.char_indices() {
            if !['C', 'G', 'A', 'U'].contains(&c) {
                return Err(i);
            }
        }
        Ok(RNA {
            rna: rna.to_string(),
        })
    }
}
