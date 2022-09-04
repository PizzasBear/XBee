#![no_std]

#[allow(unused_imports)]
#[macro_use]
extern crate xbee_derive;

// use bitflags::bitflags;
// use core::ops;

pub mod frames;
pub mod stream;
pub mod zdo;
pub mod zha;

pub use stream::{Endianness, InnerData, ReadStream, WriteStream};

pub trait Cluster {
    const PROFILE_ID: ProfileId;
    const CLUSTER_ID: ClusterId;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, InnerData)]
#[repr(transparent)]
pub struct IeeeAddress(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, InnerData)]
#[repr(transparent)]
pub struct NetworkAddress(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, InnerData)]
#[repr(transparent)]
pub struct Endpoint(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, InnerData)]
#[repr(transparent)]
pub struct ClusterId(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, InnerData)]
#[repr(transparent)]
pub struct ProfileId(pub u16);

impl IeeeAddress {
    pub const COORDINATOR: Self = Self(0);
    pub const UNKNOWN: Self = Self(!0);
}

impl NetworkAddress {
    pub const COORDINATOR: Self = Self(0);
    pub const UNKNOWN: Self = Self(0xfffe);
}

impl ProfileId {
    pub const ZIGBEE_DEVICE: Self = Self(0);
}

impl ClusterId {}

impl Endpoint {
    pub const ZIGBEE_DEVICE_OBJECT: Self = Self(0);
}

#[repr(C)]
pub struct Xbee;

impl Xbee {}
