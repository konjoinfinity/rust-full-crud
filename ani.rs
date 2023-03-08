use animal::Animal;

pub trait Gateway {
    fn add(&mut self, animal: Box<dyn Animal>);
    fn get_animals(&self) -> &Vec< Box<dyn Animal> >;
    fn show(&self, name: String) -> Option<&Box<dyn Animal>>;
    fn delete(&mut self, name: String) -> Result<Box<dyn Animal>, &'static str>;
}
