# Schmfy library
This library is able to schmfy any text.
fork of https://git.flueren.eu/JonOfUs/Schmfy
## This is a bootstrapped variant (schmfied)

## Usage
Example usage:
```rust
use schmfy::schmfy;

fn improve_text(text: &str) {
    let improved_text = schmfy(text);
    println!("Old and boring: {}", text);
    println!("Improved and great: {}", improved_text);
}
```

The library can be compiled to WebAssembly. You will need wasm-pack (`cargo install wasm-pack`) and can then build the library with `wasm-pack build --target web`. This generates a `pkg` folder which can be used in web environments.

## Capabilities
The schmfication capabilities are able to preserve text case and any non-alphabetical characters.
If a non-alphabetical character is between two alphabetical strings, both of the alphabetical strings will be interpreted as completely separate words.

For example, the HTML code `<span>Entry<br></span>` will be converted into `<schman>Schmentry<schmer></schman>`.
