# Binary Storage Test

Test storing logs in different binary formats to see the trade-off between time and speed. This is probably shit.

```
> generated 500,000 logs in 10.954s, 40.0 MB in RAM
serde_json: 3.809s, 167.2 MB
postcard: 3.305s, 81.8 MB
bincode: 5.379s, 88.2 MB
our_serialization: .348s, 80.7 MB
our_serialization compressed: 22.646s, 62.3 MB ( this is so slow because of compression library, compression level can be adjusted and threaded better )
```
