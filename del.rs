use gateway::Gateway;
use use_case::Delete;
use animal::Animal;

pub struct DeleteImp<'a> {
    data_provider: &'a mut dyn Gateway
}

impl<'a> DeleteImp<'a> {
    pub fn new(data_provider: &'a mut dyn Gateway) -> Self {
        DeleteImp {data_provider}
    }
}

impl<'a> Delete for DeleteImp<'a> {
    fn execute(&mut self, name: String) -> Result<Box<dyn Animal>, &'static str> {
        self.data_provider.delete(name)
    }
}

#[cfg(test)]

mod test {

    use ::memory::Memory;
    use animal::monkey::Monkey;
    use use_case::Add;
    use use_case::add::AddImp;
    use super::DeleteImp;
    use use_case::Delete;
    use use_case::Get;
    use use_case::get::GetImp;
    use presenter::cli::Cli;

    #[test]
    fn test_delete() {
        let mut mem = Memory::new();
        let cli = Cli::new();

        let m1 = Monkey::new(String::from("Mojo 1"));
        let m2 = Monkey::new(String::from("Mojo 2"));
        let m3 = Monkey::new(String::from("Mojo 3"));

        {
            let mut adder = AddImp::new(&mut mem);
            adder.execute(Box::new(m1));
            adder.execute(Box::new(m2));
            adder.execute(Box::new(m3));
        }

        {
            let mut deleter = DeleteImp::new(&mut mem);
            let res = deleter.execute("Mojo 2".to_string());

            assert_eq!(res.unwrap().get_name(), &"Mojo 2".to_string());
        }

        let getter = GetImp::new(&mem, &cli);
        let animals = getter.execute();

        assert_eq!(getter.execute().len(), 2);
        assert_eq!(animals[0].get_name(), &"Mojo 1".to_string());
        assert_eq!(animals[1].get_name(), &"Mojo 3".to_string());
    }
    #[test]
    fn test_delete_get_error() {
        let mut mem = Memory::new();
        let cli = Cli::new();

        let m1 = Monkey::new(String::from("Mojo 1"));

        {
            let mut adder = AddImp::new(&mut mem);
            adder.execute(Box::new(m1));
        }

        {
            let mut deleter = DeleteImp::new(&mut mem);

            if let Err(e) = deleter.execute("Mojo 2".to_string()) {
                assert_eq!(e, "Cannot found Animal");
            }

        }

        let getter = GetImp::new(&mem, &cli);
        assert_eq!(getter.execute().len(), 1);
    }
}
