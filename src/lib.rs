#![no_std]
#![forbid(unsafe_code)]

//! nano-mesh-telemetry: a library for embedded-nano-mesh nodes and serial interfaces to provide a
//! unified telemetry type structure (e.g. CO2, Temperature, and other telemetry types) across
//! sensors and mesh members. The library provides a format for serialization/deserialization to and
//! from the embedded-nano-mesh packet type
