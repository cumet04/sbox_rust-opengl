extern crate gl;
use self::gl::types::*;

use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;

extern crate image;
use image::GenericImageView;

mod window;
use window::Window;

mod shader;
use shader::Shader;

extern crate cgmath;
use cgmath::{perspective, vec3, Deg, Matrix4};

fn main() {
    let mut window = Window::new("", 800, 600);

    let shader = Shader::new("src/shaders/shader.vs", "src/shaders/shader.fs");

    let vao = unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        vao
    };

    let vbo = {
        let vertices: [f32; 44] = [
            // positions, colors, texture coords, normal
            0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0,// top right
            0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,0.0, 0.0, 1.0, // bottom right
            -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,0.0, 0.0, 1.0, // bottom left
            -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0,0.0, 0.0, 1.0, // top left
        ];

        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );
        }
        vbo
    };

    let ebo = {
        let indices = [
            0, 1, 3, // first Triangle
            1, 2, 3, // second Triangle
        ];

        let mut ebo = 0;
        unsafe {
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &indices[0] as *const i32 as *const c_void,
                gl::STATIC_DRAW,
            );
        }
        ebo
    };

    unsafe {
        let stride = 11 * mem::size_of::<GLfloat>() as GLsizei;
        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        // color attribute
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);
        // texture coord attribute
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (6 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(2);
        // normal attribute
        gl::VertexAttribPointer(
            3,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (8 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(3);
    }

    // load and create a texture
    // -------------------------
    let texture = {
        let mut texture = 0;
        // load image, create texture and generate mipmaps
        let img = image::open(&Path::new("resources/textures/container.jpg"))
            .expect("Failed to load texture");
        let data = img.to_bytes();
        let (width, height) = img.dimensions();
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        texture
    };

    window.render_loop(|| unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::BindTexture(gl::TEXTURE_2D, texture);
        shader.use_program();

        shader.set_vec3(&CString::new("lightColor").unwrap(), 0.0, 1.0, 1.0);
        shader.set_vec3(&CString::new("lightPos").unwrap(), 0.0, 0.0, 2.0);

        shader.set_mat4(
            &CString::new("model").unwrap(),
            &Matrix4::from_angle_x(Deg(-55.)),
        );
        shader.set_mat4(
            &CString::new("view").unwrap(),
            &Matrix4::from_translation(vec3(0., 0., -3.)),
        );
        shader.set_mat4(
            &CString::new("projection").unwrap(),
            &perspective(Deg(45.0), 800 as f32 / 600 as f32, 0.1, 100.0),
        );

        gl::BindVertexArray(vao);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
    });

    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ebo);
    }
}
