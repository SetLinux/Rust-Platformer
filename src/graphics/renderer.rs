#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: na::Vector3<f32>,
    pub tex_coord: na::Vector3<f32>,
}
impl Vertex {
    //Just a constructor
    pub fn new(_pos: na::Vector3<f32>, _texcoord: na::Vector3<f32>) -> Vertex {
        Vertex {
            position: _pos,
            tex_coord: _texcoord,
        }
    }
}
pub struct Renderer {
    pub vao_id: gl::types::GLuint,
    pub vbo_id: gl::types::GLuint,
    pub ebo_id: gl::types::GLuint,
    pub verticesmap: std::collections::HashMap<gl::types::GLuint, Vec<Vertex>>,
    pub indices: [u32; 3612],
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            vao_id: 0,
            ebo_id: 0,
            vbo_id: 0,
            verticesmap: std::collections::HashMap::new(),
            indices: [0; 3612],
        }
    }
    pub fn init(&mut self) {
        const SPRITESIZE: i32 = std::mem::size_of::<Vertex>() as i32 * 4;
        const MAXSPRITES: i32 = 601;
        const BUFFERSIZE: i32 = SPRITESIZE * MAXSPRITES;
        const INDICESBUFFERSIZE: usize = MAXSPRITES as usize * 6;
        println!("supposde buffer size : {:?}",INDICESBUFFERSIZE);
        let mut m_offset = 0;
        for i in 0..INDICESBUFFERSIZE {
            if i % 6 == 0 || i == 0 {
                self.indices[i + 0] = m_offset + 0;
                self.indices[i + 1] = m_offset + 1;
                self.indices[i + 2] = m_offset + 3;
                self.indices[i + 3] = m_offset + 1;
                self.indices[i + 4] = m_offset + 2;
                self.indices[i + 5] = m_offset + 3;
                m_offset += 4;
            }
        }
        unsafe {
            gl::GenBuffers(1, &mut self.ebo_id);
            gl::GenBuffers(1, &mut self.vbo_id);
            gl::GenVertexArrays(1, &mut self.vao_id);
            gl::BindVertexArray(self.vao_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo_id);


            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (std::mem::size_of::<i32>() as i32 * (INDICESBUFFERSIZE as i32))
                    as gl::types::GLsizeiptr,
                self.indices.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );
            gl::BufferData(
                gl::ARRAY_BUFFER,
                BUFFERSIZE as gl::types::GLsizeiptr,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                false as gl::types::GLboolean,
                (std::mem::size_of::<Vertex>()) as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                false as gl::types::GLboolean,
                (std::mem::size_of::<Vertex>()) as gl::types::GLsizei,
                std::mem::size_of::<na::Vector3<f32>>() as *const std::ffi::c_void,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
    pub fn begin(&mut self) {}

    //ignore this Hell rh
    pub fn submit(&mut self, vert_slice: &mut [Vertex], tex_id: gl::types::GLuint) {
        for x in 0..vert_slice.len() {
            if self.verticesmap.contains_key(&tex_id) {
                match self.verticesmap.get_mut(&tex_id) {
                    Some(res) => {
                        res.push(vert_slice[x].clone());
                    }
                    None => (println!("OK RHIS IS BAD")),
                }
            } else {
                let mut temp_vec: Vec<Vertex> = vec![];
             //   temp_vec.reserve(6 * 1001);
                temp_vec.push(vert_slice[x].clone());
                self.verticesmap.insert(
                    tex_id,
                    temp_vec
                );
            }
        }
    }
    pub fn end(&mut self) {
        let mut offset: usize = 0;
        for (key, value) in &mut self.verticesmap {
            unsafe {
                gl::BindVertexArray(self.vao_id);
                gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_id);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo_id);

                gl::BindTexture(gl::TEXTURE_2D, (*key) as u32);
                gl::GetError();

                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    (0 * std::mem::size_of::<Vertex>()) as gl::types::GLintptr,
                    (value.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
                    value.as_ptr() as *const std::ffi::c_void,
                );
                gl::DrawElements(
                    gl::TRIANGLES,
                    (value.len() as i32 / 4 * 6) as gl::types::GLsizei,
                    gl::UNSIGNED_INT,
                    0  as *const std::ffi::c_void,
                );

                offset = offset+ value.len()  ;

                value.clear();
            }
        }
    }
}
