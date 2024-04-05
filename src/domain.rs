use std::ops;

#[derive(Debug)]
pub struct KnownAcronym {
    pub abbreviation: String,
    pub definition: String,
}

impl KnownAcronym {
    pub fn new(abbreviation: String, definition: String) -> Self {
        KnownAcronym {
            abbreviation: abbreviation.to_uppercase(),
            definition,
        }
    }
}

impl Clone for KnownAcronym {
    fn clone(&self) -> Self {
        KnownAcronym {
            abbreviation: self.abbreviation.clone(),
            definition: self.definition.clone(),
        }
    }
}

pub struct TargetAcronym {
    pub value: String,
}

impl TargetAcronym {
    pub fn new(value: String) -> Self {
        TargetAcronym {
            value: value.to_uppercase(),
        }
    }
}

#[derive(Debug)]
pub struct AcronymResult {
    pub acronym: KnownAcronym,
    pub matched_range: ops::Range<usize>,
}

pub fn lookup_acronym(
    target: &TargetAcronym,
    known_acronyms: Vec<KnownAcronym>,
) -> Option<Vec<AcronymResult>> {
    let results = known_acronyms
        .iter()
        .filter_map(|acronym| {
            let abbreviation = &acronym.abbreviation;
            let value = &target.value;

            if abbreviation.contains(value) {
                let start = abbreviation.find(value).unwrap();
                let range = start..(start + value.len());
                let result = AcronymResult {
                    acronym: acronym.clone(),
                    matched_range: range,
                };
                return Some(result);
            }
            return None;
        })
        .collect::<Vec<AcronymResult>>();

    if results.is_empty() {
        return None;
    }
    return Some(results);
}
