extern crate rand;
extern crate ed25519_dalek;

use rustler::{NifResult, NifStruct};

rustler::atoms! { error, ok, }

// Needed to map to the Elixir struct - failing the correct name means
// you will be finding a generic struct in Elixir:
// like: `%{__struct__: <WrongName>, value: 1}`
#[derive(NifStruct)]
#[module = "SolanaED.CustomStruct"]
struct MyStruct {
    value: i32
}

#[rustler::nif]
fn hello() -> NifResult<MyStruct> {
    let ret_value = MyStruct { value: 1 };

    Ok(ret_value)
}


rustler::init!("Elixir.SolanaED", [hello]);
