use std::error::Error;

use wire::{Input, Output};

mod wire;

#[link(wasm_import_module = "host")]
extern "C" {
    fn get_input_size() -> i32;
    fn get_input(ptr: i32);
    fn set_output(ptr: i32, size: i32);
}


fn main() -> Result<(), Box<dyn Error>> {
    let input_buf = read_host_input();

    println!("input_buf = {:?}", input_buf);

    let input: Input = serde_json::from_slice(&input_buf).map_err(|e| {
        eprintln!("ser: {e}");
        e
    })?;

    println!("input = {:?}", input);

    let names: Vec<String> = (0..input.num).map(|_idx| input.name.clone()).collect();

    // returning a value is done by allocating a new variable in linear memory and 
    // storing the pointer and size.
    let output = Output { names };
    let serialized = serde_json::to_vec(&output).map_err(|e| {
        eprintln!("de: {e}");
        e
    })?;
    let size = serialized.len() as i32;
    let ptr = serialized.as_ptr();
    std::mem::forget(ptr);

    unsafe {
        set_output(ptr as i32, size);
    }

    Ok(())
}


fn read_host_input() -> Vec<u8> {
    let mem_size = unsafe { get_input_size() };
    println!("mem_size: {}", mem_size);

    let mut buf: Vec<u8> = Vec::with_capacity(mem_size as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(ptr);

    let input_ptr = unsafe { get_input(ptr as i32) };
    println!("input_ptr: {:?}", input_ptr);

    let input_buf = unsafe {
        Vec::from_raw_parts(ptr, mem_size as usize, mem_size as usize)
    };
    println!("input_buf: {:?}", input_buf);

    input_buf
}