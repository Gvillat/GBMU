use crate::registers::{Bits16, Bus};
use crate::Registers;
use memory::{Async, Memory};
use shared::Error;
use std::future::Future;
use std::pin::Pin;

type Getter<T> = Pin<Box<dyn Future<Output = Result<(T, u8), Error>>>>;

pub(crate) trait GetAt<T> {
    fn get_at(self, memory: Memory, area: Bits16) -> Getter<T>;
}

impl GetAt<u8> for Registers {
    fn get_at(self, memory: Memory, area: Bits16) -> Getter<u8> {
        let address = self.borrow().get(area);
        Box::pin(memory.get::<u8>(address))
    }
}

impl GetAt<u16> for Registers {
    fn get_at(self, memory: Memory, area: Bits16) -> Getter<u16> {
        let address = self.borrow().get(area);
        Box::pin(memory.get::<u16>(address))
    }
}
