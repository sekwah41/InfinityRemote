pub trait Protocol {
    fn send(&self, message: &str);
    fn receive(&self) -> String;
}