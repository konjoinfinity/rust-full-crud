pub struct Response {
    name: String
}

impl Response {
    pub fn new(name: String) -> Self {
        Response {name}
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
