use gateway::Gateway;
use use_case::Get;
use animal::Animal;
use presenter::Presenter;
use response::Response;

pub struct GetImp<'a> {
    data_provider: &'a dyn Gateway,
    presenter: &'a dyn Presenter
}

impl<'a> Get for GetImp<'a> {
    fn execute(&self) -> &Vec< Box<dyn Animal> > {
        let animals = self.data_provider.get_animals();

        for animal in animals {
            let res = Response::new(animal.get_name().to_string());
            self.presenter.present(res);
        }

        &animals
    }
}

impl<'a> GetImp<'a> {
    pub fn new(data_provider: &'a dyn Gateway, presenter: &'a dyn Presenter) -> Self {
        GetImp { data_provider, presenter }
    }
}

#[cfg(test)]
mod test {

    use ::memory::Memory;
    use super::GetImp;
    use ::use_case::Get;
    use ::use_case::add::AddImp;
    use ::use_case::Add;
    use ::animal::duck::Duck;
    use presenter::cli::Cli;

    #[test]
    fn test_get_and_add() {
        let mut mem = Memory::new();
        let cli = Cli::new();
        let d1 = Duck::new("Daffy".to_string());
        let d2 = Duck::new("Donald".to_string());

        {
            let mut adder = AddImp::new(&mut mem);
            adder.execute(Box::new(d1));
            adder.execute(Box::new(d2));
        }
        {
            let getter = GetImp::new(&mem, &cli);
            assert_eq!(getter.execute().len(), 2);
        }
    }
}
