pub trait Device {
    fn connect(&self);
    fn disconnect(&self);
}