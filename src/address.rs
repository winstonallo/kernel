struct InvalidVirtualAddress;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct VirtualAddress(usize);

impl VirtualAddress {
    pub const fn new(address: usize) -> Self {
        match Self::try_new(address) {
            Ok(address) => address,
            Err(_) => panic!("Virtual address must be sign-extended in bits 48 to 64"),
        }
    }

    const fn try_new(address: usize) -> Result<Self, InvalidVirtualAddress> {
        match Self::new_truncate(address) {
            validated_address if validated_address.0 == address => Ok(validated_address),
            _ => Err(InvalidVirtualAddress {}),
        }
    }

    /// Performs sign extension to make the address canonical. This overwrites
    /// bits 48 to 64.
    const fn new_truncate(address: usize) -> Self {
        Self(((address << 16) as i64 >> 16) as usize)
    }
}
