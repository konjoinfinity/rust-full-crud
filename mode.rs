pub mod cli;
use response::Response;

pub trait Presenter {
    fn present(&self, res: Response);
}
