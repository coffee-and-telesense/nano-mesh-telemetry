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
use core::fmt::Debug;
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

impl TelemetryPacket {
    /// Deserialize a `TelemetryPacket` from `PacketDataBytes`
    #[must_use]
    pub fn from_packet_data(data: &PacketDataBytes) -> Option<Self> {
        Self::from_song(data.as_slice()).ok()
    }

    /// Serialize a `TelemetryPacket` to `PacketDataBytes`
    #[must_use]
    pub fn to_packet_data(&self) -> Option<PacketDataBytes> {
        let mut buf = [0u8; 32];
        let written = self.song_size();
        self.to_song(&mut buf).ok()?;
        PacketDataBytes::from_slice(&buf[..written]).ok()
    }
}

/// Node statistics structure
#[derive(Clone, Copy, Debug, Default, FromSong, ToSong, SongSize)]
pub struct NodeStatsPacket {
    /// UNIX epoch timestamp
    pub epoch: u32,
    /// Increments once per reboot, stored in non-volatile memory
    pub reboot_count: u8,
    /// Number of transmit attempts failed (`SendError::SendingQueueIsFull`)
    pub tx_fail: u16,
    /// Packets dropped because the receive queue was full (`NodeUpdateError::is_receive_queue_full`)
    pub rx_drop: u16,
    /// New non-duplicate packets successfully received
    pub rx_useful: u16,
    /// Receive queue overflow events (proxy for simultaneous receptions)
    pub rx_overlap: u16,
    /// Transit queue full events (`NodeUpdateError::is_transit_queue_full`)
    pub queue_full: u16,
    /// Bad packets received (CRC failures etc.)
    pub rx_bad: u16,
    /// Number of transmit attempts failed due to timeout (`SpecialSendError::Timeout`)
    pub tx_timeout: u16,
    /// Number of transmit attempts failed due to sending queue full (`SpecialSendError::SendingQueueIsFull`)
    pub special_tx_fail: u16,
    /// Online nodes observed
    pub num_online_nodes: u8,
    /// Total nodes known
    pub num_total_nodes: u8,
    /// Channel utilization 0.0–1.0
    pub channel_util: f32,
    /// Air utilization transmit 0.0–1.0
    pub air_util_tx: f32,
}

/// Sensor measurements structure
#[derive(Debug, Clone, Copy, FromSong, ToSong, SongSize)]
pub struct SensorPacket {
    /// UNIX epoch timestamp
    pub epoch: u32,
    /// The ID of the sensor used
    pub sensor_id: SensorId,
    /// The count of measurements
    pub count: u8,
    /// The measurements
    pub measurements: [Measurement; MAX_MEASUREMENTS],
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
            epoch: 0,
            sensor_id: SensorId::Unknown,
            count: 0,
            measurements: [Measurement {
                kind: MeasurementKind::Unknown,
                value: 0.0,
            }; MAX_MEASUREMENTS],
        };
        assert_eq!(pkt.song_size(), 31, "SensorPacket must be 31 wire bytes");
    }

    #[test]
    fn node_stats_wire_size() {
        let pkt = NodeStatsPacket::default();
        assert_eq!(pkt.song_size(), 31, "NodeStatsPacket must be 31 wire bytes");
    }

    #[test]
    fn telemetry_packet_fits_content_size() {
        let pkt = TelemetryPacket::Sensor(SensorPacket {
            epoch: 0,
            sensor_id: SensorId::Unknown,
            count: 0,
            measurements: [Measurement {
                kind: MeasurementKind::Unknown,
                value: 0.0,
            }; MAX_MEASUREMENTS],
        });
        assert!(
            pkt.song_size() <= 32,
            "TelemetryPacket must fit in 32-byte CONTENT_SIZE, got {}",
            pkt.song_size()
        );
    }
    #[test]
    fn telemetry_packet_sensor_roundtrip() {
        let original = TelemetryPacket::Sensor(SensorPacket {
            sensor_id: SensorId::Unknown,
            count: 2,
            epoch: 1_735_689_601,
            measurements: [
                Measurement {
                    kind: MeasurementKind::Temperature,
                    value: 23.5,
                },
                Measurement {
                    kind: MeasurementKind::Co2,
                    value: 412.0,
                },
                Measurement {
                    kind: MeasurementKind::Unknown,
                    value: 0.0,
                },
                Measurement {
                    kind: MeasurementKind::Unknown,
                    value: 0.0,
                },
                Measurement {
                    kind: MeasurementKind::Unknown,
                    value: 0.0,
                },
            ],
        });

        let bytes = original.to_packet_data().expect("serialization failed");
        let decoded = TelemetryPacket::from_packet_data(&bytes).expect("deserialization failed");

        match decoded {
            TelemetryPacket::Sensor(s) => {
                assert_eq!(s.epoch, 1_735_689_601);
                assert_eq!(s.count, 2);
                assert_eq!(s.measurements[0].kind, MeasurementKind::Temperature);
                assert!((s.measurements[0].value - 23.5).abs() < f32::EPSILON);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn telemetry_packet_node_stats_roundtrip() {
        let original = TelemetryPacket::NodeStats(NodeStatsPacket {
            epoch: 1_735_689_601,
            reboot_count: 3,
            tx_fail: 10,
            rx_drop: 2,
            rx_useful: 100,
            rx_overlap: 1,
            queue_full: 0,
            rx_bad: 5,
            tx_timeout: 1,
            special_tx_fail: 1,
            num_online_nodes: 4,
            num_total_nodes: 6,
            channel_util: 0.15,
            air_util_tx: 0.08,
        });

        let bytes = original.to_packet_data().expect("serialization failed");
        let decoded = TelemetryPacket::from_packet_data(&bytes).expect("deserialization failed");

        match decoded {
            TelemetryPacket::NodeStats(ns) => {
                assert_eq!(ns.epoch, 1_735_689_601);
                assert_eq!(ns.reboot_count, 3);
                assert_eq!(ns.rx_useful, 100);
                assert!((ns.channel_util - 0.15).abs() < f32::EPSILON);
            }
            _ => panic!("wrong variant"),
        }
    }
}
