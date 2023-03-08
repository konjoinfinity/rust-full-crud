pub mod monkey;
pub mod duck;

pub trait Animal {
    fn get_name(&self) -> &String;
}
