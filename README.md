# lynx-scene-generator

## Usage

To run you gotta specify the port:
```bash
carg run -- 8080
```

You can also run with generating images feature enabled. It will generate locally stored image of the map on every request in the `./out` directory.
```bash
cargo run --features rendering-images -- 8080
```

The structure of GET request is as follows:
```json
{
	"seed" : "test",
	"width": 128,
	"height" : 128
}
```

## Example maps
Ground and trees rendered separately:

![map](https://i.imgur.com/asvewVI.png)
![map](https://i.imgur.com/NVpAh4C.png)

