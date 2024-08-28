use serde::ser::SerializeStruct;

#[derive(Clone, Debug)]
pub struct KnownAcronym {
    pub abbreviation: String,
    pub definition: String,
}

impl KnownAcronym {
    pub fn new(abbreviation: &str, definition: &str) -> Self {
        KnownAcronym {
            abbreviation: abbreviation.trim().to_uppercase(),
            definition: definition.trim().to_string(),
        }
    }
}

pub struct TargetAcronym {
    pub value: String,
}

impl TargetAcronym {
    pub fn new(value: &str) -> Self {
        TargetAcronym {
            value: value.trim().to_uppercase(),
        }
    }
}

#[derive(Debug)]
pub struct AcronymResult {
    pub acronym: KnownAcronym,
}

impl serde::Serialize for AcronymResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut result = serializer.serialize_struct("", 2)?;
        result.serialize_field("Acronym", &self.acronym.abbreviation)?;
        result.serialize_field("Definition", &self.acronym.definition)?;
        result.end()
    }
}

pub fn lookup_acronym(
    target: &TargetAcronym,
    known_acronyms: Vec<KnownAcronym>,
) -> Option<Vec<AcronymResult>> {
    let results = known_acronyms
        .iter()
        .filter_map(|acronym| {
            let abbreviation = &acronym.abbreviation;
            let target_value = &target.value;

            if abbreviation.contains(target_value) {
                Some(AcronymResult {
                    acronym: acronym.clone(),
                })
            } else {
                None
            }
        })
        .collect::<Vec<AcronymResult>>();

    if results.is_empty() {
        None
    } else {
        Some(results)
    }
}
