#![no_std]
#![forbid(unsafe_code)]

//! nano-mesh-telemetry: a library for embedded-nano-mesh nodes and serial interfaces to provide a
//! unified telemetry type structure (e.g. CO2, Temperature, and other telemetry types) across
//! sensors and mesh members. The library provides a format for serialization/deserialization to and
//! from the embedded-nano-mesh packet type

use core::marker::PhantomData;

use bitsong::{
    ConstSongSizeImplFromConstSongSize, ConstSongSizeValue, FromSong, FromSongError, HasSongSize,
    SongSize, ToSong, ToSongError,
};
use embedded_nano_mesh::PacketDataBytes;
use num_enum::{FromPrimitive, IntoPrimitive};

/// A `Sensor` connected to a node
pub trait Sensor {
    /// Setup a sensor
    fn setup(&mut self);
}

/// The `Telemetry` a `Sensor` returns
pub trait Telemetry: FromSong + ToSong {
    /// Serialize the telemetry into `PacketDataBytes`
    fn serialize(&self) -> PacketDataBytes;
    /// Deserialize the telemetry from `PacketDataBytes`
    fn deserialize(data: PacketDataBytes) -> Self;
}

/// An enum to discriminate between telemetry types
#[repr(u8)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    IntoPrimitive,
    FromPrimitive,
    FromSong,
    ToSong,
    SongSize,
)]
pub enum TelemetryType {
    /// `Environment` telemetry type
    #[num_enum(default)]
    Environment,
    /// `AirQuality` telemetry type
    AirQuality,
    /// `NodeStats` telemetry type
    NodeStats,
}

/// An enum to discriminate sensor names
#[repr(u8)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    IntoPrimitive,
    FromPrimitive,
    FromSong,
    ToSong,
    SongSize,
)]
pub enum SensorId {
    /// An unknown sensor
    #[num_enum(default)]
    Unknown = 0xFF,
}

/// Telemetry packets must be a maximum of 32 bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromSong, ToSong, SongSize)]
pub struct TelemetryPacket {
    kind: TelemetryType,
    sensor: SensorId,
    data: [u8; 30],
}
