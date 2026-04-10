#![no_std]
#![forbid(unsafe_code)]

//! nano-mesh-telemetry: a library for embedded-nano-mesh nodes and serial interfaces to provide a
//! unified telemetry type structure (e.g. CO2, Temperature, and other telemetry types) across
//! sensors and mesh members. The library provides a format for serialization/deserialization to and
//! from the embedded-nano-mesh packet type

use embedded_nano_mesh::PacketDataBytes;

/// A `Sensor` connected to a node
pub trait Sensor {
    /// Setup a sensor
    fn setup(&mut self);
}

/// The `Telemetry` a `Sensor` returns
pub trait Telemetry {
    /// Serialize the telemetry into `PacketDataBytes`
    fn serialize(&self) -> PacketDataBytes;
    /// Deserialize the telemetry from `PacketDataBytes`
    fn deserialize(data: PacketDataBytes) -> Self;
}
