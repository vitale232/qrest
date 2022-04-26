# qrest

A small CLI tool to query [ArcGIS REST API](https://developers.arcgis.com/rest/services-reference/enterprise/query-feature-service-layer-.htm) services, implemented in Rust. The server response is returned as pretty JSON.

Said another way, Query REST, aka qrest. Pronounced crest ⛰️

Usage example:

```shell
./qrest https://gisservices.its.ny.gov/arcgis/rest/services/NYS_Place_Points/FeatureServer/0/query --where "County = 'Essex' AND PlaceType = 'Incorporated Town'" --count
# Outputs:
# {
#   "count": 18
# }
```

## Compiling

Brief instructions follow. For more detail, see [Screwtape's Notepad's article](https://zork.net/~st/jottings/rust-windows-and-debian.html), which this is based on.

Use [cargo](https://github.com/rust-lang/cargo) to build for your system.

```shell
cargo build --release
```

### To cross-compile from Debian to 64-bit Windows,

```
# One-time setup
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
sudo apt install mingw-w64
```

Then configure the linker `~/.cargo/config`:
```shell
[target.x86_64-pc-windows-gnu]
linker = "/usr/bin/x86_64-w64-mingw32-gcc"
```

Once setup, you can build with:
```shell
cargo build --target x86_64-pc-windows-gnu --release
```
