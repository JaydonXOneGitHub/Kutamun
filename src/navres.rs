pub enum NavRes<T, E> {
    OK(T),
    Error(E)
}

impl<T, E> NavRes<T, E> {
    pub fn is_error(&self) -> bool {
        return match &self {
            Self::Error(_) => true,
            _ => false
        };
    }

    pub fn is_value(&self) -> bool {
        return !self.is_error();
    }

    pub fn ok(self) -> Option<T> {
        return match self {
            Self::OK(t) => Option::Some(t),
            Self::Error(_) => Option::None
        };
    }

    pub fn err(self) -> Option<E> {
        return match self {
            Self::Error(e) => Option::Some(e),
            Self::OK(_) => Option::None
        };
    }
}