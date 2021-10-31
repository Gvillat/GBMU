use super::Text;
use iced_wgpu::{Container, Row};
use std::fmt::Display;

pub struct Register {
    text_size: u16,
    name_width: usize,
    data_width: usize,
}

impl Register {
    pub fn new(text_size: u16, name_width: usize, data_width: usize) -> Self {
        Self {
            text_size,
            name_width,
            data_width,
        }
    }

    pub fn render<'a, Message: 'a, Str, Data>(&self, name: Str, data: Data) -> Row<'a, Message>
    where
        Str: Into<String> + Display,
        Data: Into<String> + Display,
    {
        let data = format!("{:^width$}", data, width = self.data_width);
        let data = Text::new(data).light(self.text_size);
        let data = Container::new(data);
        let name = format!("{:<width$}", name, width = self.name_width);
        let name = Text::new(name).bold(self.text_size);
        Row::new().push(name).push(data)
    }
}
