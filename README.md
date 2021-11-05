# Sensirion SGP41 Driver
Platform agnostic Rust device driver for Sensirion SGP41.

[![Build status][workflow-badge]][workflow]  
[![Crates.io Version][crates-io-badge]][crates-io]  
[![Crates.io Downloads][crates-io-download-badge]][crates-io-download]  
![No Std][no-std-badge]  

This library provides an embedded `no_std` driver for the [Sensirion SGP41 sensor](https://www.sensirion.com/en/environmental-sensors/gas-sensors/sgp41/). This driver was built using [embedded-hal](https://docs.rs/embedded-hal/) traits. The implementaion is based on [scd4x](https://github.com/hauju/scd4x-rs.git) and [sgp40-rs](https://github.com/mjaakkol/sgp40-rs.git).

## Sensirion SGP41

The SGP41 is Sensirion’s new VOC and NOx sensor designed as digital smart switch and regulation unit for air treatment devices such as air purifiers. 

Further information: [Datasheet Gas Sensors SGP41](https://www.sensirion.com/fileadmin/user_upload/customers/sensirion/Dokumente/9_Gas_Sensors/Datasheets/Sensirion_Gas_Sensors_Datasheet_SGP41.pdf)

## Usage

See an example using `linux-embedded-hal` in `examples/linux.rs`.
```bash
cargo run --example linux
```

## Development Status

The driver is in an development and testing state.  
VOC index scale and NOx index scale not implemented yet.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.