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

/// UBIOS memory map table.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C, align(8))]
pub struct MemoryMapTable {
    /// Table name, "mem_map_tbl\0".
    pub name: [u8; 11],
    /// Field type.
    pub field_type: u8,
    /// Field length.
    pub field_length: u32,
    /// Total size, including this table header.
    pub total_size: u32,
    /// Table version.
    pub version: u8,
    /// Reserved field 3.
    pub _reserved_0x21: [u8; 3],
    /// Remaining size of this table.
    pub remaining_size: u32,
    /// Checksum of this structure.
    pub checksum: u32,
}

/// UBIOS memory map region table.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C, align(8))]
pub struct MemoryMapRegionTable {
    /// Base address of the memory region (Must be the Physical Address).
    pub base: u64,
    /// Size of the memory region.
    pub size: u64,
    /// Memory Region Type.
    pub region_type: MemoryRegionType,
    /// Memory Reliability.
    pub reliability: MemoryReliability,
    /// Hot Plug Support.
    pub hot_plug: bool,
    /// Reserved field 5.
    pub _reserved: [u8; 5],
}

/// UBIOS Memory Region Type
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C, align(1))]
pub enum MemoryRegionType {
    /// Unused Memory.
    Free = 0,
    /// Efficient Memory, as Free Memory after READ.
    Data = 1,
    /// Receiver Code.
    Code = 2,
    /// Using between Reporter and Receiver.
    Shared = 3,
    /// Reserved Memory (Cannot Use).
    Reserved = 4,
    /// Cannot Use.
    Disabled = 5,
    /// Device Memory, cannot be added to normal Memory.
    Device = 6,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C, align(1))]
pub enum MemoryReliability {
    /// Most of the DDRs are normal reliability.
    Normal = 0,
    /// Higher reliability.
    High = 1,
    /// Lower reliability.
    Low = -1,
}

// TODO struct CallIdServiceTable

// TODO struct UvbTable

// TODO struct IsaCallTable

// TODO struct NotifyInfoTable

// TODO unit tests on: 1. structure field offsets 2. #[derive]'d features
