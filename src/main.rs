mod components;
mod graphics;
mod movement;
mod physics;
mod player_system;
mod window_manager;
mod map_system;
extern crate cutec2_sys;
extern crate nalgebra as na;
extern crate ncollide2d;
extern crate specs;
extern crate serde;
use serde::{Serialize, Deserialize};
#[macro_use]
extern crate specs_derive;
extern crate image;
use glfw::{Action, Context, Key};
use specs::{Builder, RunNow};
use std::cell::RefCell;
use std::rc::Rc;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;


fn main() {
    let mut world = specs::World::new();
    world.register::<components::Transform>();
    world.register::<components::Sprite>();
    world.register::<components::Movement>();
    world.register::<components::Collider>();
    world.register::<components::Player>();
    let (mut glfwer, mut window) = window_manager::create_window(1000, 1000, "easy".to_string());
    window_manager::make_context_current(&mut window);
    let vertshdr = graphics::shader::shader_from_source(
        &std::ffi::CString::new(include_str!("base.vert")).unwrap(),
        gl::VERTEX_SHADER,
    )
    .unwrap();
    let fragshdr = graphics::shader::shader_from_source(
        &std::ffi::CString::new(include_str!("base.frag")).unwrap(),
        gl::FRAGMENT_SHADER,
    )
    .unwrap();
    let prgoram_id = graphics::shader::create_program(vertshdr, fragshdr);
    unsafe {
        gl::UseProgram(prgoram_id);
    }
    let file = File::open(&Path::new("assets/map.json")).unwrap();
    println!("Opened file");
   // let reader = BufReader::new(file);
    let tex_id = graphics::texture::Texture::new("assets/derb.png".to_string()).tex_id;
    graphics::texture::Texture::new("assets/sheet.png".to_string()).tex_id;
    /*
    let deserialized : serde_json::Value  = serde_json::from_reader(reader).unwrap();
    println!("THIS IS THE FILER {:?}",(deserialized["layers"][0]["data"].as_array().unwrap().len()));
    let the_map_array = (deserialized["layers"][0]["data"].as_array().unwrap());


    for x in 0 .. the_map_array.len() {
        if(the_map_array.get(x).unwrap()  != 0) {
            let ypos = (x as f32 / 32.0).floor();
            let remaningxpos = (x as f32 / 32.0).floor() * 32.0;
            let xpos = x as f32 - remaningxpos;
            println!("the positions is  : {:?} , {:?}",xpos,ypos);

            world.create_entity().with(components::Transform {
                position: (na::Vector3::<f32>::new(xpos * 32.0, ypos * 32.0, 0.0)),
                rotation: 0.0,
                scale: (na::Vector3::<f32>::new(128.0, 128.0, 0.0))
            }).with(components::Sprite { tex_id: tex_id }).build();
        }
    }
    */
    map_system::load_map(&file,&na::Vector2::<f32>::new(000.0,1000.0),&mut world,32,32,32);
    //Renderer System Initialization
    let mut renderer = graphics::renderer::Renderer::new();
    let mut hello_world = graphics::render_system::RenderSystem::new(&mut renderer);
    let mut movsystem = movement::MovementSystem;
    let mut rcs = graphics::render_system::RenderCommandsSystem {
        m_renderer: &mut hello_world,
    };
    let physicssystem = Rc::new(RefCell::new(physics::PhysicsSystem { m_bodies: vec![] }));
    let mut swept_movement = player_system::SweptMovement {
        physicssytem: physicssystem.clone(),
    };
    let mut player_system = player_system::PlayerSystem {
        moveright: false,
        moveleft: false,
        moveup: false,
        movedown: false,
        physicssystem: physicssystem.clone(),
    };


    world
        .create_entity()
        .with(components::Transform {
            position: na::Vector3::<f32>::new(144.90194, 535.9981, 0.0),
            rotation: 0.0,
            scale: na::Vector3::<f32>::new(16.0, 16.0, 1.0),
        })
        .with(components::Sprite { tex_id: tex_id ,coords : na::Vector4::<f32>::new(0.0,0.0,0.0,0.0)})
        .with(components::Movement {
            velocity: na::Vector3::<f32>::new(1.0, 0.0, 0.0),
        })
        .with(components::Player {
            health: 0.0,
            vel: na::Vector3::<f32>::new(0.0, 0.0, 0.0),
            jump: false,
        })
        .build();

    world.maintain();
    physicssystem.try_borrow_mut().unwrap().run_now(&world.res);
    rcs.m_renderer.init();

    while !window.glwin.should_close() {
        glfwer.poll_events();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(0.1, 0.0, 0.1, 1.0);
            gl::Enable(gl::MULTISAMPLE);
            {
                if glfw::ffi::glfwGetKey(window.glwin.window_ptr(), glfw::ffi::KEY_A)
                    == glfw::ffi::PRESS
                {
                    player_system.moveleft = true;
                } else if glfw::ffi::glfwGetKey(window.glwin.window_ptr(), glfw::ffi::KEY_D)
                    == glfw::ffi::PRESS
                {
                    player_system.moveright = true;
                }

                if glfw::ffi::glfwGetKey(window.glwin.window_ptr(), glfw::ffi::KEY_W)
                    == glfw::ffi::PRESS
                {
                    player_system.moveup = true;
                }

                if glfw::ffi::glfwGetKey(window.glwin.window_ptr(), glfw::ffi::KEY_S)
                    == glfw::ffi::PRESS
                {
                    player_system.movedown = true;
                }
            }


            rcs.m_renderer.renderer.begin();
            player_system.run_now(&world.res);
            swept_movement.run_now(&world.res);
            movsystem.run_now(&world.res);

            physicssystem.try_borrow_mut().unwrap().run_now(&world.res);

            rcs.run_now(&world.res);
            rcs.m_renderer.render();
            rcs.m_renderer.flush();
        }
        window.glwin.swap_buffers();
    }
}
