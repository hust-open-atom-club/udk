//! UBIOS system information tables (Section 11.3).

use crate::{CallId, NotifyId, UserId};

/// For general header field.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
struct GeneralHeader {
    /// Table name, "table name".
    pub name: [u8; 16],
    /// Total size, including this table header.
    pub size: u32,
    /// Version field.
    pub version: u8,
    /// Reserved field.
    pub _reserved: [u8; 3],
    /// Remaining size of all table.
    pub remain_size: u32,
    /// Checksum and padding.
    pub checksum: u32,
}

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

/// UBIOS memory region type.
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

/// UBIOS Call ID service object description header field.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct CallIdServiceOdHeader(GeneralHeader);

/// UBIOS Call ID service group usage type.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum CallIdServiceGroupUsageType {
    /// Not supported, not currently in use, but may be used in
    /// the future to dynamically disable the Call ID service.
    Unsupported = 0,
    /// Use ISA to make the call.
    IsaCall = 1,
    /// Use UVB.
    Uvb = 2,
    /// Reserved.
    Reserved = 3,
    /// Undefined, typically used by device firmware.
    Undefined = 4,
}

/// UBIOS Call ID service object description group struct.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct CallIdServiceOdGroup {
    /// The User ID of the component to which this group belongs.
    pub owner: UserId,
    /// Indicates the usage type of this group.
    pub usage: CallIdServiceGroupUsageType,
    /// When the number of items with the same usage exceeds 1,
    /// you need to specify the specific usage to be used.
    ///
    /// If UVB is used, it is represented as the index of UVB.
    pub index: u8,
    /// Indicates which Call ID the component supports.
    pub call_id: [CallId; 512],
    /// Instruct the specific component to forward the message.
    pub forwarder: UserId,
}

/// UBIOS Call ID service ub.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct CallIdServiceUb {
    /// Refer to [CallIdServiceOdGroup::usage].
    pub usage: CallIdServiceGroupUsageType,
    /// Refer to [CallIdServiceOdGroup::index].
    pub index: u8,
    /// Refer to [CallIdServiceOdGroup::forwarder].
    pub forwarder: UserId,
}

/// UBIOS Call ID service table.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct CallIdServiceTable {
    /// The Call ID Service Object Description Header.
    pub header: CallIdServiceOdHeader,
    /// The Call ID Service Object Description Group.
    pub groups: [CallIdServiceOdGroup; 512],
    /// The Call ID Service Ub.
    pub ub: CallIdServiceUb,
}

/// UBIOS Uvb object description header field.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct UvbOdHeader(GeneralHeader);

/// UBIOS Uvb Object Description Member Field.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct UvbOdMember {
    /// When acquiring the window, read back the previously written User ID after a delay of `X` ms.
    pub delay: u32,
    /// A window has a Windows description.
    pub wd: [UvbWindowDescript; 512],
}

/// UBIOS Ubv window description.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct UvbWindowDescript {
    /// Used to obtain the system physical address of this window (write the occupant's User ID to this address).
    pub obtain: u64,
    /// The system physical address of the UVB window.
    ///
    /// The window area must be of the shared type in memory mapping.
    pub address: u64,
    /// The system physical address of the specific space within this window (if it exists).
    ///
    /// This space must be of the shared type in memory mapping.
    pub buffer: u64,
    /// The size of the buffer. If a buffer exists, this is required.
    ///
    /// If the buffer does not exist or exists but is 0, it will be ignored.
    pub size: u32,
}

/// UBIOS Uvb table.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct UvbTable {
    /// The Uvb header field.
    pub header: UvbOdHeader,
    /// The Uvb member field.
    pub member: [UvbOdMember; 512],
}

/// The ISA call header field.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IsaCallHeader(GeneralHeader);

/// The ISA call buffer type.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum IsaCallBufferType {
    Uvb = 2,
}

/// UBIOS ISA call table
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IsaCallTable {
    ///  The ISA Call header field.
    pub header: IsaCallHeader,
    /// Indicates the buffer type used when transferring data via the ISA call.
    pub buffer_type: IsaCallBufferType,
    /// The index of the buffer, which has the same meaning as the index in the [CallIdServiceOdGroup::index].
    pub buffer_index: u8,
}

/// UBIOS Notify Info object description header field.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct NotifyInfoOdHeader(GeneralHeader);

/// UBIOS Notify Info ring buffer data field.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct NotifyInfoRingBufferData {
    /// Notify ID
    pub notify_id: NotifyId,
    /// Excludes the Notify ID and data length fields.
    pub length: u32,
    /// The data is determined by the Information ID.
    pub raw_data: [u8; 512],
}

/// UBIOS Notify Info ring buffer struct.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct NotifyInfoRingBuffer {
    /// The starting position of the ring buffer, used for transmitting Notify ID information,
    /// is the system physical address.
    pub head: u64,
    /// The end of the ring buffer used for transmitting Notify ID information is the system physical address.
    pub tail: u64,
    /// The address storing the read position indicates the start of valid Notify ID Information.
    pub read_pos: u64,
    /// The address storing the write position indicates the end of valid Notify ID Information
    /// and the start of the empty area.
    pub write_pos: u64,
    /// The data of ring buffer.
    pub data: NotifyInfoRingBufferData,
}

/// UBIOS Notify Info IRQ clear struct.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct NotifyInfoIrqClear {
    /// The address of IRQ clear data.
    pub addr: u64,
    /// The mask of IRQ clear data.
    pub mask: u64,
    /// The value of IRQ clear data.
    pub value: u64,
}

/// UBIOS Notify Info IRQ.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct NotifyInfoIrq {
    /// The interrupt number used to send signals to the OS.
    pub number: u32,
    /// Information used to clear the interrupt status.
    pub clear: NotifyInfoIrqClear,
}

/// UBIOS Notify Info table.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct NotifyInfoTable {
    /// The nofity info od header field.
    pub header: NotifyInfoOdHeader,
    /// Information about the ring buffer
    pub ring_buffer: NotifyInfoRingBuffer,
    /// The index of UVB in virtual_bus, used for transmitting Notify ID information.
    pub uvb_index: u8,
    /// Interrupt request information.
    pub irq: NotifyInfoIrq,
}

// TODO unit tests on: 1. structure field offsets 2. #[derive]'d features
