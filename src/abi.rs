#[derive(Debug, Default, PartialEq)]
pub enum Abi {
    #[default]
    SystemV64,
    Uefi,
}
