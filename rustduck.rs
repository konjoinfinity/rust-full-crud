use Animal;

pub struct Duck {
    name: String
}

impl Animal for Duck {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl Duck {
    pub fn new(name: String) -> Self {
        Duck { name }
    }
}
