use uuid::Uuid;

#[derive(Clone)]
pub enum FnName {
    Name(String),
    Anonymous(Uuid),
}

impl FnName {
    pub fn is_named(&self) -> bool {
        matches!(self, FnName::Name(_))
    }

    pub fn is_anonymous(&self) -> bool {
        matches!(self, FnName::Anonymous(_))
    }
}

impl ToString for FnName {
    fn to_string(&self) -> String {
        match self {
            FnName::Name(name) => name.clone(),
            FnName::Anonymous(uuid) => "<anonymous>".into(),
        }
    }
}
