use getset::Getters;

#[derive(Debug, Getters)]
pub struct Car {
    #[get = "pub"]
    name: String,

    #[get = "pub"]
    color: String,
}

impl Car {
    pub fn new(name: String, color: String) -> Self {
        Self {
            name: name,
            color: color
        }
    }
} 