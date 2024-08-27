pub mod rand;

pub trait CommandHandler {
    fn execute(&self);
}
