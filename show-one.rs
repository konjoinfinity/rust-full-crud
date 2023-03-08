use gateway::Gateway;
use use_case::Show;
use animal::Animal;

pub struct ShowImp<'a> {
    data_provider: &'a dyn Gateway
}

impl<'a> ShowImp<'a> {
    pub fn new(data_provider: &'a dyn Gateway) -> Self {
        ShowImp { data_provider }
    }
}

impl<'a> Show for ShowImp<'a> {
    fn execute(&self, name: String) -> Option<&Box<dyn Animal>> {
        self.data_provider.show(name)
    }
}

#[cfg(test)]
mod test {

    use ::memory::Memory;
    use ::animal::monkey::Monkey;
    use ::use_case::add::AddImp;
    use ::use_case::Add;
    use super::ShowImp;
    use ::use_case::Show;

    #[test]
    fn test_show() {
        let mut mem = Memory::new();
        let m1 = Monkey::new(String::from("Mojo 1"));
        let m2 = Monkey::new(String::from("Mojo 2"));

        {
            let mut adder = AddImp::new(&mut mem);
            adder.execute(Box::new(m1));
            adder.execute(Box::new(m2));
        }

        let shower = ShowImp::new(&mem);
        let fm1 = shower.execute("Mojo 1".to_string());

        assert_eq!(fm1.unwrap().get_name(), &"Mojo 1".to_string());
    }

    #[test]
    #[should_panic]
    fn test_return_none() {
        let mut mem = Memory::new();
        let m1 = Monkey::new(String::from("Mojo 1"));

        {
            let mut adder = AddImp::new(&mut mem);
            adder.execute(Box::new(m1));
        }

        let shower = ShowImp::new(&mem);
        let other = shower.execute("Other monkey".to_string());
        
        other.unwrap();
    }
}
