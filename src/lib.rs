/// SAS data step variable and format model for SigmaOS statistical bridge.

#[derive(Debug, Clone, PartialEq)]
pub enum SasType {
    Numeric,
    Character(u32),
}

#[derive(Debug, Clone)]
pub struct SasVariable {
    pub name: String,
    pub vtype: SasType,
    pub label: Option<String>,
    pub format: Option<String>,
}

impl SasVariable {
    pub fn numeric(name: &str) -> Self {
        Self {
            name: name.into(),
            vtype: SasType::Numeric,
            label: None,
            format: None,
        }
    }

    pub fn character(name: &str, len: u32) -> Self {
        Self {
            name: name.into(),
            vtype: SasType::Character(len),
            label: None,
            format: None,
        }
    }

    pub fn with_label(mut self, l: &str) -> Self {
        self.label = Some(l.into());
        self
    }
    pub fn with_format(mut self, f: &str) -> Self {
        self.format = Some(f.into());
        self
    }

    pub fn sas_declaration(&self) -> String {
        match &self.vtype {
            SasType::Numeric => format!("    {} 8.", self.name),
            SasType::Character(len) => format!("    $ {} ${}.", self.name, len),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_declaration_aaa() {
        let v = SasVariable::numeric("age");
        assert!(v.sas_declaration().contains("age"));
    }

    #[test]
    fn test_character_declaration_contains_len_aaa() {
        let v = SasVariable::character("name", 32);
        assert!(v.sas_declaration().contains("32"));
    }

    #[test]
    fn test_with_label_aaa() {
        let v = SasVariable::numeric("bmi").with_label("Body Mass Index");
        assert_eq!(v.label.as_deref(), Some("Body Mass Index"));
    }

    #[test]
    fn test_with_format_aaa() {
        let v = SasVariable::numeric("date").with_format("DATE9.");
        assert_eq!(v.format.as_deref(), Some("DATE9."));
    }
}
