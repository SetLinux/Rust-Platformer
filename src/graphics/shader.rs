pub fn shader_from_source(
    source: &std::ffi::CStr,
    kind: gl::types::GLenum,
) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }
    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
        buffer.extend([b' '].iter().cycle().take(len as usize));
        let error: std::ffi::CString = unsafe { std::ffi::CString::from_vec_unchecked(buffer) };
        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }
        return Err(error.to_string_lossy().into_owned());
    }
    Ok(id)
}

pub fn create_program(
    vertex_shader: gl::types::GLuint,
    fragment_shader: gl::types::GLuint,
) -> gl::types::GLuint {
    let program_id = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(program_id, vertex_shader);
        gl::AttachShader(program_id, fragment_shader);
        gl::LinkProgram(program_id);
    }
    program_id
}
