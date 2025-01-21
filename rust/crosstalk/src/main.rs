use wasmtime::*;

fn get_wasm_talk(instance: &Instance, store: &mut impl AsContextMut) -> Result<String> {
    // Get talk ptr and len getter functions by calling our wasm instance
    let talk_ptr_func = instance.get_typed_func::<(), u32>(&mut *store, "get_talk_ptr")?;
    let talk_len_func = instance.get_typed_func::<(), u32>(&mut *store, "get_talk_len")?;

    // Get the actual talk ptr and len by calling the getter function
    let talk_ptr = talk_ptr_func.call(&mut *store, ())? as usize;
    let talk_len = talk_len_func.call(&mut *store, ())? as usize;

    // Get access to our shared linear memory 
    let Some(mem) = instance.get_memory(&mut *store, "memory") else {
        return Err(Error::msg("Failed to get shared liniear memory"));
    };

    // Use the talk ptr and len to convert that section of memory to the final talk string
    let data = mem.data(store)[talk_ptr..(talk_ptr+talk_len)].to_vec();
    let talk = String::from_utf8(data)?;
    
    Ok(talk)
}

fn main() -> Result<()> {
    // Create a new WebAssembly instance
    let mut store: Store<()> = Store::default();
    let module = Module::from_file(
        store.engine(),
        "public/wasm32-unknown-unknown/debug/crosstalk.wasm")?;
    let instance = Instance::new(&mut store, &module, &[])?;

    // Make our wasm talk
    let wasm_talk = get_wasm_talk(&instance, &mut store)?;

    // Print our wasmTalk to stdout
    println!("{}",wasm_talk);

    Ok(())
}
