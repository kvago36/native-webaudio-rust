#![feature(portable_simd)]

use std::alloc::{Layout, alloc, dealloc};
use std::simd::Simd;
use std::simd::f32x4;
use std::simd::num::SimdFloat;

unsafe extern "C" {
    unsafe fn console_log(ptr: *const u8, len: usize);
}

// Вспомогательная функция для логирования строк
fn log(message: &str) {
    unsafe {
        console_log(message.as_ptr(), message.len());
    }
}

#[unsafe(no_mangle)]
pub fn alloc_i16(len: usize) -> *mut i16 {
    unsafe {
        let layout = Layout::array::<i16>(len).unwrap();
        let ptr = alloc(layout) as *mut i16;
        if ptr.is_null() {
            panic!("Allocation failed");
        }
        ptr
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dealloc_i16(ptr: *mut i16, len: usize) {
    unsafe {
        let layout = Layout::array::<i16>(len).unwrap();
        dealloc(ptr as *mut u8, layout);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn alloc_f32(len: usize) -> *mut f32 {
    unsafe {
        let layout = Layout::array::<f32>(len).unwrap();
        let ptr = alloc(layout) as *mut f32;
        if ptr.is_null() {
            panic!("Allocation failed");
        }
        ptr
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dealloc_f32(ptr: *mut f32, len: usize) {
    unsafe {
        let layout = std::alloc::Layout::array::<f32>(len).unwrap();
        dealloc(ptr as *mut u8, layout);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn custom_alloc(len: usize) -> *mut u8 {
    let mut buffer = Vec::with_capacity(len);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer); // не даем Rust освободить память
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn process_audio_simd(input_ptr: *const f32, output_ptr: *mut i16, byte_len: usize) {
    const LANES: usize = 4;

    let float_slice = unsafe { std::slice::from_raw_parts(input_ptr, byte_len) };

    let int_slice = unsafe { std::slice::from_raw_parts_mut(output_ptr as *mut i16, byte_len) };

    for (i, chunk) in float_slice.chunks_exact(LANES).enumerate() {
        let input_chunk = f32x4::from_slice(chunk);
        let min = Simd::splat(-1.0);
        let max = Simd::splat(1.0);
        let clamped = input_chunk.simd_clamp(min, max);
        let scaled = clamped * f32x4::splat(i16::MAX as f32);
        let ints = scaled.cast::<i16>();

        ints.copy_to_slice(&mut int_slice[i * LANES..(i + 1) * LANES]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::ptr::NonNull;

    use lazy_static::lazy_static;
    use std::alloc::{alloc, Layout};
    use std::ptr;
    use std::sync::Mutex;

    const LEN: usize = 12;

    const RESULT: [i16; LEN] = [
        23733, 5042, 29313, 10534, 18605, 406, 32708, 14112, 9014, 21754, 26849, 4836,
    ];

    const INPUT: [f32; LEN] = [
        0.7243, 0.1539, 0.8946, 0.3215, 0.5678, 0.0124, 0.9982, 0.4307, 0.2751, 0.6639, 0.8194,
        0.1476,
    ];

    struct RawPtr<T>(NonNull<T>);

    unsafe impl<T> Send for RawPtr<T> {}
    unsafe impl<T> Sync for RawPtr<T> {}

    lazy_static! {
        static ref OUTPUT_PTR: Mutex<RawPtr<i16>> = unsafe {
            let output_layout = Layout::array::<i16>(LEN).unwrap();
            let output_ptr = alloc(output_layout) as *mut i16;

            Mutex::new(RawPtr(NonNull::new(output_ptr).expect("allocation failed")))
        };
    }

    lazy_static! {
        static ref INPUT_PTR: Mutex<RawPtr<f32>> = unsafe {
            let input_layout = Layout::array::<f32>(LEN).unwrap();
            let input_ptr = alloc(input_layout) as *mut f32;

            ptr::copy_nonoverlapping(INPUT.as_ptr(), input_ptr, LEN);

            Mutex::new(RawPtr(NonNull::new(input_ptr).expect("allocation failed")))
        };
    }

    #[test]
    fn test_process_audio_simd() {
        let input_ptr = INPUT_PTR.lock().unwrap().0.as_ptr();
        let output_ptr = OUTPUT_PTR.lock().unwrap().0.as_ptr();

        let _ = process_audio_simd(input_ptr, output_ptr, LEN);

        let result = unsafe { std::slice::from_raw_parts(output_ptr, LEN) };
        let mut ints = vec![];
        ints.extend_from_slice(&RESULT);

        assert_eq!(ints, result)
    }
}