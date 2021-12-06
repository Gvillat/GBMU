pub mod apu;
pub mod interface;
pub mod saver_apu;

pub use interface::Apu;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
