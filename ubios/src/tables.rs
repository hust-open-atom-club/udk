//! UBIOS system information tables (Section 11.3).

/// UBIOS information index table (the Root table).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RootTable {
    /// Table name, "root_table\0".
    pub name: [u8; 16],
    /// Total size, including this table header.
    pub total_size: u32,
    /// Table version.
    pub version: u8,
    /// Reserved field 1.
    pub _reserved_0x21: [u8; 3],
    /// Remaining size of this table.
    pub remaining_size: u32,
    /// Checksum of this structure.
    pub checksum: u32,
    /// Count of Object Description (OD) files.
    pub count: u16,
    /// Reserved field 2.
    pub _reserved_0x34: [u8; 6],
}

// TODO struct MemoryMapTable

// TODO struct CallIdServiceTable

// TODO struct UvbTable

// TODO struct IsaCallTable

// TODO struct NotifyInfoTable

// TODO unit tests on: 1. structure field offsets 2. #[derive]'d features
