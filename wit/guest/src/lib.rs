wit_bindgen::generate!({
    world: "lib",
    path: "../wit",
});

struct Lib;

impl Guest for Lib {
    fn to_upper(input: String) -> String {
        input.to_uppercase()
    }
}
export!(Lib);
