mod cool_card;

pub use cool_card::*;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Cool {
    pub name: String,
    pub version: String,
    pub description: String,
}

impl Cool {
    pub fn new(
        name: impl AsRef<str>,
        version: impl AsRef<str>,
        description: impl AsRef<str>,
    ) -> Self {
        Cool {
            name: name.as_ref().to_string(),
            version: version.as_ref().to_string(),
            description: description.as_ref().to_string(),
        }
    }
}
