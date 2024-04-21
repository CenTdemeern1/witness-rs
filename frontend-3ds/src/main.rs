#![feature(allocator_api)]

use citro3d::{attrib::{Format, Info, Register}, buffer::Primitive, math::{AspectRatio, ClipPlanes, Matrix4, Projection}, shader::{Library, Program}, Instance};
use ctru::{linear::LinearAllocator, prelude::*, services::gfx::TopScreen3D};

// fn load_example_shader_program() -> (Library, Program) {
//     let vshader = std::fs::read("romfs:/shaders/projection_vcolor.v.pica.bin").unwrap();
//     let library = Library::from_bytes(&vshader).unwrap();
//     let entrypoint = library.get(0).unwrap();
//     let program = Program::new(entrypoint).unwrap();
//     (library, program)
// }

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let _console = Console::new(gfx.bottom_screen.borrow_mut());
    let _romfs = ctru::services::romfs::RomFS::new().unwrap();
    let mut gpu = Instance::new().unwrap();
    let top_screen = TopScreen3D::from(&gfx.top_screen);
    let (mut left_eye, mut _right_eye) = top_screen.split_mut();

    // Create a render target
    let top_screen_render_target = gpu.render_target(400, 240, left_eye, None).unwrap();

    // Load shader program
    // let (shader_library, shader_program) = load_example_shader_program();
    let vshader = std::fs::read("romfs:/shaders/projection_vcolor.v.pica.bin").unwrap();
    let shader_library = Library::from_bytes(&vshader).unwrap();
    let entrypoint = shader_library.get(0).unwrap();
    let shader_program = Program::new(entrypoint).unwrap();

    gpu.render_frame_with(|gpu| {
        // Select target
        gpu.select_render_target(&top_screen_render_target).unwrap();

        // Bind shader program
        gpu.bind_program(&shader_program);

        // Bind uniform: projection matrix
        let projection_uniform_index = shader_program.get_uniform("projection").unwrap();
        let projection = Projection::perspective(
            1.22173, // 70° as radians
            AspectRatio::TopScreen,
            ClipPlanes {
                near: 0.01,
                far: 100.
            }
        );
        let projection_matrix: Matrix4 = projection.into();
        gpu.bind_vertex_uniform(projection_uniform_index, projection_matrix);

        // Bind attribute info: 3xFloat Position, 3xFloat Color
        let mut attr_info = Info::new();
        attr_info.add_loader(Register::new(0).unwrap(), Format::Float, 3).unwrap(); // Position
        attr_info.add_loader(Register::new(1).unwrap(), Format::Float, 3).unwrap(); // Color
        gpu.set_attr_info(&attr_info);

        let mut vbinfo = citro3d::buffer::Info::new();
        let vbo: Box<[f32; 18], LinearAllocator> = Box::new_in([
            0., 1., 0.,
            1., 0., 0.,

            -1., -1., 0.,
            0., 1., 0.,

            1., -1., 0.,
            0., 0., 1.,
        ], LinearAllocator);
        let vbo_slice = vbinfo.add(vbo.as_slice(), &attr_info).unwrap();

        gpu.draw_arrays(Primitive::Triangles, vbo_slice);
    });

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

    drop(shader_library)
}
