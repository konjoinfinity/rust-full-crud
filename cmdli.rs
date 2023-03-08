use presenter::Presenter;
use response::Response;

pub struct Cli {}

impl Cli {
    pub fn new() -> Self {
        Cli {}
    }
}

impl Presenter for Cli {

    fn present(&self, res: Response) {
        println!("{}", res.get_name());
    }
}
