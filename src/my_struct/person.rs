use crate::my_trait::country::Country;

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u8
}

impl Country for Person {
    fn country(&self) -> String {
        format!("Hello {}", self.name)
    }
}

