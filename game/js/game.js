register_plugin = function (importObject) {
    // make macroquad_js_get_random_buffer() function available to call from rust
    importObject.env.macroquad_js_get_random_buffer = macroquad_js_get_random_buffer;
}

// register this plugin in miniquad, required to make plugin's functions available from rust
miniquad_add_plugin({register_plugin});

function macroquad_js_get_random_buffer(length) {
    const myArray = new Uint8Array(length);
    crypto.getRandomValues(myArray);
    return js_object(myArray);
}