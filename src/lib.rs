#![no_std]
#![forbid(unsafe_code)]

//! nano-mesh-telemetry: a library for embedded-nano-mesh nodes and serial interfaces to provide a
//! unified telemetry type structure (e.g. CO2, Temperature, and other telemetry types) across
//! sensors and mesh members. The library provides a format for serialization/deserialization to and
//! from the embedded-nano-mesh packet type

use bitsong::{
    ConstSongSizeImplFromConstSongSize, ConstSongSizeValue, FromSong, FromSongError, HasSongSize,
    SongDiscriminant, SongSize, ToSong, ToSongError,
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
    Unknown,
}

/// What kind of measurement is being transmitted
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
#[repr(u8)]
pub enum MeasurementKind {
    /// Unknown measurement kind
    #[num_enum(default)]
    Unknown,
    /// Degrees Celsius
    Temperature,
    /// Percent relative humidity
    Humidity,
    /// Hectopascals
    Pressure,
    /// ohms
    GasResistance,
    /// 0–500 (Bosch BSEC)
    Iaq,
    /// Parts per million
    Co2,
    /// Meters per second
    WindSpeed,
    /// Degrees 0–360
    WindDirection,
    /// Millimeters per hour
    Rainfall1h,
    /// Millimeters per hour
    Rainfall24h,
    /// Volts
    Voltage,
    /// Milliamps
    Current,
    /// Micrograms per cubed meter
    Pm1Standard,
    /// Micrograms per cubed meter
    Pm25Standard,
    /// Micrograms per cubed meter
    Pm10Standard,
    /// Meters per second squared
    AccelX,
    /// Meters per second squared
    AccelY,
    /// Meters per second squared
    AccelZ,
    /// Meters per second
    WindGust,
    /// Meters per second
    WindLull,
    /// Millimeters
    RainfallTotal,
    /// Micrograms per cubed meter
    Pm1Env,
    /// Micrograms per cubed meter
    Pm25Env,
    /// Micrograms per cubed meter
    Pm10Env,
    /// Particle count
    Particles03um,
    /// Particle count
    Particles05um,
    /// Particle count
    Particles10um,
    /// Particle count
    Particles25um,
    /// Particle count
    Particles50um,
    /// Particle count
    Particles100um,
}

/// A struct representing measurements
#[derive(Debug, Clone, Copy, FromSong, ToSong, SongSize)]
pub struct Measurement {
    /// The kind of measurement
    pub kind: MeasurementKind,
    /// The measurement taken
    pub value: f32,
}

/// The maximum number of measurements in a `SensorPacket`
pub const MAX_MEASUREMENTS: usize = 5;

/// Telemetry packets must be a maximum of 32 bytes
#[derive(Debug, Clone, Copy, FromSong, ToSong, SongSize)]
#[song(discriminant(TelemetryType = u8))]
pub enum TelemetryPacket {
    /// Telemetry from a sensor
    Sensor(SensorPacket),
    /// Telemetry for Node statistics
    NodeStats(NodeStatsPacket),
}

/// Node statistics structure
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, FromSong, ToSong, SongSize)]
pub struct NodeStatsPacket {
    /// Number of transmit attempts failed (`SendingQueueIsFull`)
    pub tx_fail: u16,
    /// Packets dropped because the receive queue was full
    pub rx_drop: u16,
    /// New non-duplicate packets successfully received
    pub rx_useful: u16,
    /// Receive queue overflow events (proxy for simultaneous receptions)
    pub rx_overlap: u16,
    /// Transit queue full events
    pub queue_full: u16,
    /// UNIX epoch timestamp
    pub epoch: u32,
}

/// Sensor measurements structure
#[derive(Debug, Clone, Copy, FromSong, ToSong, SongSize)]
pub struct SensorPacket {
    /// The ID of the sensor used
    pub sensor_id: SensorId,
    /// The count of measurements
    pub count: u8,
    /// The measurements
    pub measurements: [Measurement; MAX_MEASUREMENTS],
    /// UNIX epoch timestamp
    pub epoch: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measurement_wire_size() {
        let m = Measurement {
            kind: MeasurementKind::Temperature,
            value: 0.0,
        };
        assert_eq!(m.song_size(), 5, "Measurement must be 5 wire bytes");
    }

    #[test]
    fn sensor_packet_wire_size() {
        let pkt = SensorPacket {
            sensor_id: SensorId::Unknown,
            count: 0,
            measurements: [Measurement {
                kind: MeasurementKind::Unknown,
                value: 0.0,
            }; MAX_MEASUREMENTS],
            epoch: 0,
        };
        assert_eq!(pkt.song_size(), 31, "SensorPacket must be 31 wire bytes");
    }

    #[test]
    fn node_stats_wire_size() {
        let pkt = NodeStatsPacket::default();
        assert_eq!(pkt.song_size(), 14, "NodeStatsPacket must be 14 wire bytes");
    }

    #[test]
    fn telemetry_packet_fits_content_size() {
        let pkt = TelemetryPacket::Sensor(SensorPacket {
            sensor_id: SensorId::Unknown,
            count: 0,
            measurements: [Measurement {
                kind: MeasurementKind::Unknown,
                value: 0.0,
            }; MAX_MEASUREMENTS],
            epoch: 0,
        });
        assert!(
            pkt.song_size() <= 32,
            "TelemetryPacket must fit in 32-byte CONTENT_SIZE, got {}",
            pkt.song_size()
        );
    }
}
