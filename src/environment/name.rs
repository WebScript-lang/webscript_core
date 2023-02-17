use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Name {
    Name {
        name: String,
        scope_name: Option<String>,
    },
    Anonymous {
        id: Uuid,
        scope_name: Option<String>,
    },
}

impl Name {
    pub fn new(name: Option<String>, scope_name: Option<String>) -> Self {
        name.map_or_else(
            || Name::Anonymous {
                id: Uuid::new_v4(),
                scope_name: scope_name.clone(),
            },
            |name| Name::Name {
                name,
                scope_name: scope_name.clone(),
            },
        )
    }

    /// Returns true if it is a named variant
    pub fn is_named(&self) -> bool {
        matches!(self, Name::Name { .. })
    }

    /// Returns true if it is an anonymous variant
    pub fn is_anonymous(&self) -> bool {
        matches!(self, Name::Anonymous { .. })
    }

    /// Returns the name if it is a named variant or `None` otherwise
    pub fn name(&self) -> Option<String> {
        match self {
            Name::Name { name, .. } => Some(name.clone()),
            Name::Anonymous { .. } => None,
        }
    }

    pub fn codename(&self) -> String {
        match self {
            Name::Name { name, scope_name } => match scope_name {
                Some(scope_name) => format!("{scope_name}.{name}"),
                None => name.clone(),
            },
            Name::Anonymous { id, scope_name } => match scope_name {
                Some(scope_name) => format!("{scope_name}.{id}"),
                None => id.to_string(),
            },
        }
    }
}
