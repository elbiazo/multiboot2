use crate::tag_type::TagTypeId;
#[cfg(feature = "builder")]
use {
    crate::builder::traits::StructAsBytes, crate::TagType, core::convert::TryInto,
    core::mem::size_of,
};

/// If the image has relocatable header tag, this tag contains the image's
/// base physical address.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C, align(8))]
pub struct ImageLoadPhysAddr {
    typ: TagTypeId,
    size: u32,
    load_base_addr: u32,
}

impl ImageLoadPhysAddr {
    #[cfg(feature = "builder")]
    pub fn new(load_base_addr: u32) -> Self {
        Self {
            typ: TagType::LoadBaseAddr.into(),
            size: size_of::<Self>().try_into().unwrap(),
            load_base_addr,
        }
    }

    /// Returns the load base address.
    pub fn load_base_addr(&self) -> u32 {
        self.load_base_addr
    }
}

#[cfg(feature = "builder")]
impl StructAsBytes for ImageLoadPhysAddr {
    fn byte_size(&self) -> usize {
        size_of::<Self>()
    }
}

#[cfg(all(test, feature = "builder"))]
mod tests {
    use super::ImageLoadPhysAddr;

    const ADDR: u32 = 0xABCDEF;

    #[test]
    fn test_build_load_addr() {
        let tag = ImageLoadPhysAddr::new(ADDR);
        assert_eq!(tag.load_base_addr(), ADDR);
    }
}
