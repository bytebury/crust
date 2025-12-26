use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Checkbox(String);

impl Checkbox {
    pub fn checked() -> Self {
        Checkbox("on".into())
    }

    pub fn unchecked() -> Self {
        Checkbox(String::default())
    }

    pub fn is_checked(&self) -> bool {
        self.value()
    }

    pub fn is_unchecked(&self) -> bool {
        !self.is_checked()
    }

    fn value(&self) -> bool {
        self.0 == "on"
    }
}

impl From<bool> for Checkbox {
    fn from(value: bool) -> Self {
        if value {
            Checkbox::checked()
        } else {
            Checkbox::unchecked()
        }
    }
}

impl From<Checkbox> for bool {
    fn from(value: Checkbox) -> Self {
        value.value()
    }
}

impl Default for Checkbox {
    fn default() -> Self {
        Checkbox::unchecked()
    }
}
