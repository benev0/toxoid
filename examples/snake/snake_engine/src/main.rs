#![allow(improper_ctypes_definitions)]
// mod systems;
// mod components;
pub use toxoid_ffi::*;

extern "C" {
    // Main function of the dynamically linked library / Toxoid App.
    pub fn app_main();
}

fn main() {
    // Initialize Toxoid ECS initializers + default components + default sy stems.
    toxoid_ffi::init();
    
    // TODO: Move to dynamically linked lib
    // systems::init();

    // components::init();
}

#[no_mangle]
pub unsafe extern "C" fn app_init() {
    // Main function of the dynamically linked library / Toxoid App.
    app_main();

    toxoid_sokol::sokol();
    
    // Start update loop / game loop / render loop.
    // toxoid_ffi::start();
}

#[no_mangle]
pub unsafe extern "C" fn print_hello() {
    println!("HELLO WORLD 123!")
}
