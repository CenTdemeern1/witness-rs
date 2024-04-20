use citro3d::{Instance, shader::{Library, Program}};
use ctru::{prelude::*, services::gfx::TopScreen3D};

fn load_example_shader_program() -> Program {
    let vshader = std::fs::read("romfs:/shaders/projection_vcolor.v.pica.bin").unwrap();
    let library = Library::from_bytes(&vshader).unwrap();
    let entrypoint = library.get(0).unwrap();
    Program::new(entrypoint).unwrap()
}

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let _console = Console::new(gfx.bottom_screen.borrow_mut());
    let _romfs = ctru::services::romfs::RomFS::new().unwrap();
    let mut gpu = Instance::new().unwrap();
    let top_screen = TopScreen3D::from(&gfx.top_screen);
    let (mut left_eye, mut right_eye) = top_screen.split_mut();

    let shader_program = load_example_shader_program();
    gpu.bind_program(&shader_program);

    println!("Hello, World!");
    println!("Call to gameplay crate: {}", gameplay::add(1, 2));
    println!("\x1b[29;11HPress Start to exit");

    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
    }
}
