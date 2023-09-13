use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HeaderField {
    pub name: String,
    pub kind: FieldKind,
}

type HFM = HashMap<String, HeaderField>;
pub struct HeaderFieldMap(HFM);

impl std::ops::Deref for HeaderFieldMap {
    type Target = HFM;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for HeaderFieldMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for HeaderFieldMap {
    type Item = <HFM as IntoIterator>::Item;
    type IntoIter = <HFM as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl HeaderFieldMap {
    pub fn new() -> Self {
        HeaderFieldMap(HashMap::<String, HeaderField>::new())
    }

    pub fn add(mut self, prefix: &str, name: &str, display_name: &str, kind: FieldKind) -> Self {
        let key = if name == "" {
            format!("{prefix}")
        } else {
            format!("{prefix}.{name}")
        };
        self.0.insert(
            key,
            HeaderField {
                name: display_name.into(),
                kind,
            },
        );
        self
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FieldKind {
    Text,
    Branch,
    // Number,
    // Bytes,
}

pub trait GenerateHFMap {
    fn generate_hf_map(prefix: &str) -> HeaderFieldMap;
}
