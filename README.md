# SIMD Audio Processing Library

A high-performance Rust library for audio processing using SIMD (Single Instruction, Multiple Data) operations. This library provides efficient conversion from floating-point audio samples to 16-bit integers with memory management utilities designed for WebAssembly and embedded applications.

## Features

- **SIMD-Optimized Audio Processing**: Uses Rust's portable SIMD for 4x parallel processing of audio samples
- **Memory Management**: Custom allocation/deallocation functions for different data types
- **WebAssembly Ready**: Includes console logging bindings for WASM environments
- **Type Safety**: Rust's memory safety with explicit unsafe blocks where needed
- **Comprehensive Testing**: Unit tests with predefined test cases

## Requirements

- Rust nightly (requires `portable_simd` feature)
- Target platform with SIMD support (x86, ARM, WebAssembly)

## Usage

### Basic Audio Processing

The main function `process_audio_simd` converts floating-point audio samples to 16-bit integers:

```rust
use your_crate::process_audio_simd;

// Allocate input and output buffers
let input_ptr = alloc_f32(sample_count);
let output_ptr = alloc_i16(sample_count);

// Process audio data
process_audio_simd(input_ptr, output_ptr, sample_count);

// Don't forget to deallocate
dealloc_f32(input_ptr, sample_count);
dealloc_i16(output_ptr, sample_count);
```

### Memory Management

The library provides specialized allocation functions:

```rust
// Allocate memory for different types
let f32_buffer = alloc_f32(1024);     // For floating-point samples
let i16_buffer = alloc_i16(1024);     // For integer samples
let raw_buffer = custom_alloc(4096);  // For raw byte buffers

// Deallocate when done
dealloc_f32(f32_buffer, 1024);
dealloc_i16(i16_buffer, 1024);
```

## API Reference

### Core Functions

#### `process_audio_simd(input_ptr: *const f32, output_ptr: *mut i16, byte_len: usize)`

Converts floating-point audio samples to 16-bit integers using SIMD operations.

- **Parameters:**
  - `input_ptr`: Pointer to input floating-point samples
  - `output_ptr`: Pointer to output 16-bit integer buffer
  - `byte_len`: Number of samples to process
- **Behavior:**
  - Clamps input values to [-1.0, 1.0] range
  - Scales to 16-bit integer range [-32768, 32767]
  - Processes 4 samples simultaneously using SIMD

#### Memory Management Functions

- `alloc_f32(len: usize) -> *mut f32`: Allocate f32 array
- `dealloc_f32(ptr: *mut f32, len: usize)`: Deallocate f32 array
- `alloc_i16(len: usize) -> *mut i16`: Allocate i16 array
- `dealloc_i16(ptr: *mut i16, len: usize)`: Deallocate i16 array

#### Utility Functions

- `log(message: &str)`: Log string to console (WebAssembly compatible)

## Performance

The SIMD implementation processes audio samples in chunks of 4, providing significant performance improvements over scalar operations:

- **Throughput**: 4x theoretical speedup for supported operations
- **Memory Access**: Optimized for cache-friendly sequential access patterns
- **Precision**: Maintains audio quality with proper clamping and scaling

## WebAssembly Integration

This library is designed to work with WebAssembly. The `console_log` external function allows logging from WASM modules:

```javascript
// JavaScript side
    const memory = new WebAssembly.Memory({
      initial: 32,
      maximum: 64,
      shared: true,
    });

    const importObject = {
      env: {
        memory,
        console_log: (arg: string) => {
          console.log(arg);
        },
      },
    };

    WebAssembly.instantiateStreaming(
      fetch("native_webaudio_rust.wasm"),
      importObject
    ).then((module) => {
      const exports = module.instance.exports as unknown as WasmExports;

      const { alloc_f32, alloc_i16, process_audio_simd } = exports;

      const input_ptr_1 = alloc_f32(SAMPLES_COUNT);
      const input_ptr_2 = alloc_f32(SAMPLES_COUNT);
      const output_ptr = alloc_i16(SAMPLES_COUNT);

      processAudio.current = process_audio_simd;
      memoryRef.current = memory;
      pointerRef.current = {
        input_ptr_1,
        input_ptr_2,
        output_ptr,
      };
    });
```

## Testing

Run tests with:

```bash
cargo test
```

The test suite includes:
- SIMD processing verification with known input/output pairs
- Memory allocation/deallocation tests
- Thread-safe static allocation testing

### Test Data

The tests use predefined floating-point inputs and expected 16-bit integer outputs to verify correct SIMD processing behavior.

## Safety Considerations

This library uses `unsafe` code for:
- Raw memory allocation and deallocation
- SIMD operations on raw pointers
- FFI bindings for WebAssembly

All unsafe operations are contained within well-defined boundaries with proper error handling.

## Building

```bash
RUSTFLAGS="-C target-feature=+atomics,+bulk-memory" \
cargo build --release --target wasm32-unknown-unknown
```


