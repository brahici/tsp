use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Outcome {
    value_in: String,
    value_out: String,
}
impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0:20} :: {1}", self.value_in, self.value_out)
    }
}
impl Outcome {
    pub fn new(ts_str: String) -> Outcome {
        Outcome {
            value_in: ts_str.to_string(),
            value_out: "".into(),
        }
    }

    pub fn set(&mut self, out: String) -> Outcome {
        self.value_out = out;
        self.to_owned()
    }
}

#[cfg(test)]
mod test {
    use crate::outcome::Outcome;

    #[test]
    fn test_outcome_implementation() {
        let r = Outcome::new("foo".to_string()).set("bar".to_string());
        assert_eq!(format!("{r}"), "foo                  :: bar");
    }
}
