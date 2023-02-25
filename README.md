# lynx-scene-generator

## Usage

To run you gotta specify the port:
```
carg run -- 8080
```

You can also run with generating images feature enabled. It will generate locally stored image of the map on every request in the `./out` directory.
```
cargo run --features rendering-images -- 8080
```
