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

    pub fn add(mut self, prefix: &str, field_name: &str, display_name: &str) -> Self {
        self.0
            .insert(
                format!("{prefix}.{field_name}"),
                HeaderField {
                    name: display_name.into(),
                    kind: FieldKind::Text,
                },
            );
        self
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FieldKind {
    Number,
    Text,
    Bytes,
    Branch,
}

pub trait IntoHFMap {
    fn into_hf_map(self, prefix: &str) -> HeaderFieldMap;
}

pub enum Span<T> {
    Enum(Vec<T>),
    Struct(T)
}

pub trait GenerateHFMap: Sized + IntoHFMap {

    fn span() -> Span<Self>;

    fn generate_hf_map(prefix: &str) -> HeaderFieldMap {
        match Self::span() {
            Span::Enum(branches) => {
                let mut hf_map = HeaderFieldMap::new();
                for item in branches {
                    hf_map.extend(item.into_hf_map(prefix.clone()));
                }
                hf_map
            }
            Span::Struct(st) => {
                st.into_hf_map(prefix)
            }
        }
    }
}
