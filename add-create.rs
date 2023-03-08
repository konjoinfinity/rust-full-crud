use use_case::Add;
use animal::Animal;
use gateway::Gateway;

pub struct AddImp<'a> {
    data_provider: &'a mut dyn Gateway,
}

impl<'a> Add for AddImp<'a> {
    fn execute(&mut self, animal: Box<impl Animal + 'static>) {
        self.data_provider.add(animal);
    }
}

impl<'a> AddImp<'a> {
    pub fn new(data_provider: &'a mut dyn Gateway) -> Self {
        AddImp { data_provider }
    }
}

#[cfg(test)]
mod test {

    use ::memory::Memory;
    use super::AddImp;
    use ::gateway::Gateway;
    use ::use_case::add::Add;
    use ::animal::monkey::Monkey;

    #[test]
    fn test_adding() {
        let monkey = Monkey::new("Mojo".to_string());
        let mut mem = Memory::new();

        assert_eq!(mem.get_animals().len(), 0);

        {
            let mut adder = AddImp::new(&mut mem);
            adder.execute(Box::new(monkey));
        }

        assert_eq!(mem.get_animals().len(), 1);
    }
}
