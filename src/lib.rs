pub use macros::*;

pub trait HasFields {
    fn num_fields(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(HasFields)]
    struct MyForm {
        _id: u32,
        name: Option<String>,
        email: Option<String>,
    }

    #[test]
    fn functional_macros() {
        let form = MyForm {
            _id: 1,
            name: Some("John".to_string()),
            email: None,
        };
        assert!(has_fields!(&form, "name"));
        assert!(require_fields!(&form, "name").is_ok());
        assert!(require_fields!(form, "name", "email") == Err(vec!["email"]));
    }

    #[test]
    fn derive() {
        let form = MyForm {
            _id: 1,
            name: Some("John".to_string()),
            email: None,
        };
        assert_eq!(form.num_fields(), 2);
    }
}
