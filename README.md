# timing-evaluation-nanoseconds

## Run
```shell

git clone https://github.com/philipgreat/timing-evaluation-nanoseconds.git
cargo build --release
target/release/timing-test

```

## Test Result

### Linux with Dell T350 Server

```text
 ---------------OS and CPU info----------------- 

Operation system: 	linux
OS Family: 		unix
Architecture: 		x86_64
show last to prevent optimized by compiler 1769054485439559220 


---------- System call SystemTime::now() -------------

Time consumed: 		345341726 ns
Loop count: 		10000000
Time per call: 		34 ns

---------- High Resolution Time with CPU tick-------------

show last to prevent optimized by compiler 222828416 

Time consumed: 		222198601 ns
Loop count: 		10000000
Time per call: 		22 ns

====================================================
```

### Apple M1 Max


```
 ---------------OS and CPU info----------------- 

Operation system:       macos
OS Family:              unix
Architecture:           aarch64

---------- System call SystemTime::now() -------------

Time consumed:          251596000 ns
Loop count:             10000000
Time per call:          25 ns
show last to prevent optimized by compiler 1769057191205847000 


---------- High Resolution Time with CPU tick-------------

show last to prevent optimized by compiler 5478291 

Time consumed:          5480000 ns
Loop count:             10000000
Time per call:          0.548 ns

====================================================

```

