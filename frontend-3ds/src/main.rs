#![feature(allocator_api)]

use citro3d::{attrib::{Format, Info, Register}, buffer::Primitive, math::{AspectRatio, ClipPlanes, Matrix4, Projection}, shader::{Library, Program}, Instance};
use ctru::{linear::LinearAllocator, prelude::*, services::gfx::TopScreen3D};

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
    let vshader = std::fs::read("romfs:/shaders/projection_vcolor.v.pica.bin").unwrap();
    let shader_library = Library::from_bytes(&vshader).unwrap();
    let entrypoint = shader_library.get(0).unwrap();
    let shader_program = Program::new(entrypoint).unwrap();

    gpu.render_frame_with(|gpu| {
        // Select target
        println!("Trace 1: Select target");
        gpu.select_render_target(&top_screen_render_target).unwrap();

        // Bind shader program
        println!("Trace 2: Bind shader program");
        gpu.bind_program(&shader_program);

        // Bind uniform: projection matrix
        println!("Trace 3: Bind uniform");
        let projection_uniform_index = shader_program.get_uniform("projection").unwrap();
        println!("Trace 4: Bind uniform");
        let projection = Projection::perspective(
            1.22173, // 70° as radians
            AspectRatio::TopScreen,
            ClipPlanes {
                near: 0.01,
                far: 100.
            }
        );
        println!("Trace 5: Bind uniform");
        let projection_matrix: Matrix4 = projection.into();
        println!("Trace 6: Bind uniform");
        gpu.bind_vertex_uniform(projection_uniform_index, projection_matrix);

        // Bind attribute info: 3xFloat Position, 3xFloat Color
        println!("Trace 7: Bind attribute info");
        let mut attr_info = Info::new();
        println!("Trace 8: Bind attribute info");
        attr_info.add_loader(Register::new(0).unwrap(), Format::Float, 3).unwrap(); // Position
        println!("Trace 9: Bind attribute info");
        attr_info.add_loader(Register::new(1).unwrap(), Format::Float, 3).unwrap(); // Color
        println!("Trace A: Bind attribute info");
        gpu.set_attr_info(&attr_info);

        // Create vertex buffer object and draw it
        println!("Trace B: Create vertex buffer object and draw it");
        let mut vbinfo = citro3d::buffer::Info::new();
        println!("Trace C: Create vertex buffer object and draw it");
        let vbo: Box<[f32; 18], LinearAllocator> = Box::new_in([
            0., 1., 0.,
            1., 0., 0.,

            -1., -1., 0.,
            0., 1., 0.,

            1., -1., 0.,
            0., 0., 1.,
        ], LinearAllocator);
        println!("Trace D: Create vertex buffer object and draw it");
        let vbo_slice = vbinfo.add(vbo.as_slice(), &attr_info).unwrap();

        println!("Trace E: Create vertex buffer object and draw it");
        gpu.draw_arrays(Primitive::Triangles, vbo_slice);
        println!("Trace F: End");
    });

    println!("Hello, World!");
    println!("Call to gameplay crate: {}", gameplay::add(1, 2));
    println!("\x1b[29;11HPress Start to exit");

    println!("Trace 10: AppLoop");
    while apt.main_loop() {
        println!("Trace 11: VBlank");
        gfx.wait_for_vblank();

        println!("Trace 12: Input");
        hid.scan_input();
        println!("Trace 13: Check");
        if hid.keys_down().contains(KeyPad::START) {
            println!("Trace 13a: Check passed");
            break;
        }
        println!("Trace 13b: Check done");
    }

    println!("Trace 14: Drop");
    drop(shader_library);
    println!("Trace 15: End");
}
