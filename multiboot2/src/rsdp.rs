//! Module for RSDP/ACPI. RSDP (Root System Description Pointer) is a data structure used in the
//! ACPI programming interface.
//!
//! The tag that the bootloader passes will depend on the ACPI version the hardware supports.
//! For ACPI Version 1.0, a `RsdpV1Tag` will be provided, which can be accessed from
//! `BootInformation` using the `rsdp_v1_tag` function. For subsequent versions of ACPI, a
//! `RsdpV2Tag` will be provided, which can be accessed with `rsdp_v2_tag`.
//!
//! Even though the bootloader should give the address of the real RSDP/XSDT, the checksum and
//! signature should be manually verified.
//!

use crate::tag_type::TagTypeId;
use core::slice;
use core::str;
use core::str::Utf8Error;
#[cfg(feature = "builder")]
use {
    crate::builder::traits::StructAsBytes, crate::TagType, core::convert::TryInto,
    core::mem::size_of,
};

const RSDPV1_LENGTH: usize = 20;

/// This tag contains a copy of RSDP as defined per ACPI 1.0 specification.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C, align(8))]
pub struct RsdpV1Tag {
    typ: TagTypeId,
    size: u32,
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_address: u32, // This is the PHYSICAL address of the RSDT
}

impl RsdpV1Tag {
    #[cfg(feature = "builder")]
    pub fn new(
        signature: [u8; 8],
        checksum: u8,
        oem_id: [u8; 6],
        revision: u8,
        rsdt_address: u32,
    ) -> Self {
        Self {
            typ: TagType::AcpiV1.into(),
            size: size_of::<Self>().try_into().unwrap(),
            signature,
            checksum,
            oem_id,
            revision,
            rsdt_address,
        }
    }

    /// The "RSD PTR " marker signature.
    ///
    /// This is originally a 8-byte C string (not null terminated!) that must contain "RSD PTR "
    pub fn signature(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(&self.signature)
    }

    /// Validation of the RSDPv1 checksum
    pub fn checksum_is_valid(&self) -> bool {
        let bytes =
            unsafe { slice::from_raw_parts(self as *const _ as *const u8, RSDPV1_LENGTH + 8) };
        bytes[8..]
            .iter()
            .fold(0u8, |acc, val| acc.wrapping_add(*val))
            == 0
    }

    /// An OEM-supplied string that identifies the OEM.
    pub fn oem_id(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(&self.oem_id)
    }

    /// The revision of the ACPI.
    pub fn revision(&self) -> u8 {
        self.revision
    }

    /// The physical (I repeat: physical) address of the RSDT table.
    pub fn rsdt_address(&self) -> usize {
        self.rsdt_address as usize
    }
}

#[cfg(feature = "builder")]
impl StructAsBytes for RsdpV1Tag {
    fn byte_size(&self) -> usize {
        size_of::<Self>()
    }
}

/// This tag contains a copy of RSDP as defined per ACPI 2.0 or later specification.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C, align(8))]
pub struct RsdpV2Tag {
    typ: TagTypeId,
    size: u32,
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_address: u32,
    length: u32,
    xsdt_address: u64,
    // This is the PHYSICAL address of the XSDT
    ext_checksum: u8,
    _reserved: [u8; 3],
}

impl RsdpV2Tag {
    #[cfg(feature = "builder")]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        signature: [u8; 8],
        checksum: u8,
        oem_id: [u8; 6],
        revision: u8,
        rsdt_address: u32,
        length: u32,
        xsdt_address: u64,
        ext_checksum: u8,
    ) -> Self {
        Self {
            typ: TagType::AcpiV2.into(),
            size: size_of::<Self>().try_into().unwrap(),
            signature,
            checksum,
            oem_id,
            revision,
            rsdt_address,
            length,
            xsdt_address,
            ext_checksum,
            _reserved: [0; 3],
        }
    }

    /// The "RSD PTR " marker signature.
    ///
    /// This is originally a 8-byte C string (not null terminated!) that must contain "RSD PTR ".
    pub fn signature(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(&self.signature)
    }

    /// Validation of the RSDPv2 extended checksum
    pub fn checksum_is_valid(&self) -> bool {
        let bytes = unsafe {
            slice::from_raw_parts(self as *const _ as *const u8, self.length as usize + 8)
        };
        bytes[8..]
            .iter()
            .fold(0u8, |acc, val| acc.wrapping_add(*val))
            == 0
    }

    /// An OEM-supplied string that identifies the OEM.
    pub fn oem_id(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(&self.oem_id)
    }

    /// The revision of the ACPI.
    pub fn revision(&self) -> u8 {
        self.revision
    }

    /// Physical address of the XSDT table.
    ///
    /// On x86, this is truncated from 64-bit to 32-bit.
    pub fn xsdt_address(&self) -> usize {
        self.xsdt_address as usize
    }

    /// This field is used to calculate the checksum of the entire table, including both checksum fields.
    pub fn ext_checksum(&self) -> u8 {
        self.ext_checksum
    }
}

#[cfg(feature = "builder")]
impl StructAsBytes for RsdpV2Tag {
    fn byte_size(&self) -> usize {
        size_of::<Self>()
    }
}
