fn build_open_glmage(width: u32, height: u32, img: image::RgbaImage) -> gl::types::GLuint {
    let mut texture_id: gl::types::GLuint = 0;
    let rawdata = img.into_raw();
    unsafe {
        gl::GenTextures(1, &mut texture_id);

        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            rawdata.as_ptr() as *const std::ffi::c_void,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }
    texture_id
}
pub struct Texture {
    pub tex_id: gl::types::GLuint,
}
impl Texture {
    pub fn new(file: String) -> Texture {
        match image::open(file.clone()) {
            Err(err) => panic!("Could't load Texture {} {}", file, err),
            Ok(img) => {
                let img = img.rotate180();
                let img = match img {
                    image::DynamicImage::ImageRgba8(img) => img,
                    img => img.to_rgba(),
                };

                let (width, height) = img.dimensions();
                Texture {
                    tex_id: build_open_glmage(width, height, img),
                }
            }
        }
    }
}
