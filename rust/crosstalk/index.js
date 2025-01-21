async function getWasmTalk(instance) {
    // Get talk ptr and len by calling our wasm instance
    const talkPtr = instance.exports.get_talk_ptr();
    const talkLen = instance.exports.get_talk_len();
    console.log('%d', talkPtr)
    console.log('%d', talkLen)

    // Get access to our shared linear memory 
    const mem = instance.exports.memory;

    // Use the talk ptr and len to convert that section of
    // memory to the final talk string
    const talk = new TextDecoder()
        .decode( mem.buffer.slice(talkPtr, talkPtr+talkLen) );
    console.log(talk);

    return talk;
}

// Create a new WebAssembly instance
const {instance} = await WebAssembly
    .instantiateStreaming(
        fetch("wasm32-unknown-unknown/debug/crosstalk.wasm"));

// Make our wasm talk
const wasmTalk = await getWasmTalk(instance);

// Display our wasmTalk as an alert
alert( wasmTalk );

// Forces this to be treated by webpack as an ES module
export{ wasmTalk }