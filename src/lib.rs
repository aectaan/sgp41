//! This library provides an embedded `no_std` driver for the [Sensirion SGP41 sensor](https://www.sensirion.com/en/environmental-sensors/gas-sensors/sgp41/). 
//! This driver was built using [embedded-hal](https://docs.rs/embedded-hal/) traits. 
//! The implementaion is based on [scd4x](https://github.com/hauju/scd4x-rs.git).
//!

#![deny(unsafe_code)]
#![cfg_attr(not(test), no_std)]

pub mod sgp41;
pub mod commands;
pub mod error;
pub mod types;