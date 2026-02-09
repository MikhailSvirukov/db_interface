use std::io;

pub trait ChangeSections {
    fn change_items(&self) -> io::Result<()>;
}

pub trait GetItems {
    fn get_items(&self) -> io::Result<()>;
}