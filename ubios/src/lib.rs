#![no_std]

mod tables;

// TODO re-export other tables
pub use tables::RootTable;

use core::num::NonZeroU32;

/// UVB message identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MessageId(u32);

impl MessageId {
    /// Get the flag of this message ID.
    #[inline]
    pub const fn flag(self) -> MessageFlag {
        match self.0 >> 30 {
            0b11 => MessageFlag::CallId,
            0b10 => MessageFlag::Ack,
            0b01 => MessageFlag::NotifyId,
            0b00 => MessageFlag::Response,
            _ => unreachable!(),
        }
    }

    /// Get the module ID of this message ID.
    #[inline]
    pub const fn module_id(self) -> ModuleId {
        ModuleId(((self.0 >> 12) & 0xffff) as u16)
    }

    /// Internal function to read raw function or information id.
    #[inline]
    const fn function_or_information_id_raw(self) -> u16 {
        (self.0 & 0xfff) as u16
    }

    /// Converts from `MessageId` to `Option<CallId>`.
    #[inline]
    pub const fn call_id(self) -> Option<CallId> {
        match self.flag() {
            MessageFlag::CallId => Some(CallId(self)),
            _ => None,
        }
    }

    /// Returns `true` if the message ID is a `CallId`.
    #[inline]
    pub const fn is_call_id(self) -> bool {
        matches!(self.flag(), MessageFlag::CallId)
    }

    /// Converts from `MessageId` to `Option<NotifyId>`.
    #[inline]
    pub const fn notify_id(self) -> Option<NotifyId> {
        match self.flag() {
            MessageFlag::NotifyId => Some(NotifyId(self)),
            _ => None,
        }
    }

    /// Returns `true` if the message ID is a `NotifyId`.
    #[inline]
    pub const fn is_notify_id(self) -> bool {
        matches!(self.flag(), MessageFlag::NotifyId)
    }
}

/// Flags in the UVB message identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MessageFlag {
    /// UBIOS Call ID.
    CallId = 0x3,
    /// Acknowledge of UBIOS Notify ID Information.
    Ack = 0x2,
    /// UBIOS Notify ID.
    NotifyId = 0x1,
    /// Response of UBIOS Call ID Service.
    Response = 0x0,
}

/// UBIOS module identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ModuleId(u16);

/// UBIOS call identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CallId(MessageId);

impl CallId {
    /// Get the function id of current call.
    #[inline]
    pub const fn function_id(self) -> FunctionId {
        FunctionId(self.0.function_or_information_id_raw())
    }

    /// Get the module id of current call
    #[inline]
    pub const fn module_id(self) -> ModuleId {
        self.0.module_id()
    }
}

/// UBIOS notify identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NotifyId(MessageId);

impl NotifyId {
    /// Get the function id of current notify.
    #[inline]
    pub const fn function_id(self) -> FunctionId {
        FunctionId(self.0.function_or_information_id_raw())
    }

    /// Get the module id of current notify
    #[inline]
    pub const fn module_id(self) -> ModuleId {
        self.0.module_id()
    }
}

// TODO information_id(self)

/// UBIOS function identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FunctionId(u16);

// TODO pub struct InformationId

/// Non-zero UBIOS user identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UserId(NonZeroU32);

// TODO user_type(self) -> UserType
// TODO index(self) -> u32

/// UBIOS user type for UserId.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum UserType {
    /// Unspecified user.
    Unspecified = 0xff,
    /// Operating systems run in secure world.
    TrustedOs = 0x30,
    /// Operating systems run in rich world.
    RichOs = 0x20,
    /// Entities in this UBPU (Unified Bus processing unit).
    EntityThisUbpu = 0x11,
    /// Entities in other UBPUs (Unified Bus processing units).
    EntityOtherUbpu = 0x10,
    /// Baseboard Management Controller.
    Bmc = 0x0b,
    /// Basic Input/Output System.
    Bios = 0x01,
}

// TODO unit tests of the above structures, testing all of their functions.
