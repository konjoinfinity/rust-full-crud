use animal::Animal;
use gateway::Gateway;

pub struct Memory {
    animals: Vec< Box<dyn Animal> >
}

impl Memory {
    pub fn new() -> Self {
        Memory { animals: Vec::new() }
    }
}

impl Gateway for Memory {
    fn add(&mut self, animal: Box<dyn Animal>) {
        self.animals.push(animal);
    }

    fn get_animals(&self) -> &Vec< Box<dyn Animal> > {
        &self.animals
    }

    fn show(&self, name: String) -> Option<&Box<dyn Animal>> {

        for animal in self.get_animals() {
            if animal.get_name() == &name {
                return Some(animal);
            }
        }

        return None;
    }

    fn delete(&mut self, name: String) -> Result<Box<dyn Animal>, &'static str> {
        let mut i = 0;

        loop {
            if i >= self.animals.len() {
                break;
            }

            if self.animals[i].get_name() == &name {
                return Ok(self.animals.remove(i));
            }

            i = i + 1;
        }

        Err("Cannot found Animal")
    }
}

#[cfg(test)]
mod test {

    use ::animal::monkey::Monkey;
    use ::animal::duck::Duck;
    use super::Memory;
    use super::Gateway;

    #[test]
    fn test_add() {
        let mut mem = Memory::new();
        let monkey = Monkey::new(String::from("George"));
        let duck = Duck::new(String::from("Daffy"));

        assert_eq!(mem.get_animals().len(), 0);

        mem.add(Box::new(monkey));
        mem.add(Box::new(duck));

        assert_eq!(mem.get_animals().len(), 2);

        let animals = mem.get_animals();
        let mut iter = animals.iter();

        assert_eq!(iter.next().unwrap().get_name(), &"George".to_string());
        assert_eq!(iter.next().unwrap().get_name(), &"Daffy".to_string());
    }
}
