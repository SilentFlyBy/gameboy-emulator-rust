mod boot;
pub mod bus;
mod buttons;
pub mod cartridge;
pub mod constants;
pub mod cpu;
mod disassembler;
pub mod gpu;
mod interrupts;
mod ram;
pub mod register;
mod spu;
mod timer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
