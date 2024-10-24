use std::ffi::CString;
use gl::types::*;

pub fn create_shader(src: &str, kind: GLenum) -> GLuint {
    let shader = unsafe { gl::CreateShader(kind) };
    let c_str = CString::new(src.as_bytes()).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
    }

    // Vérifier les erreurs de compilation
    let mut success = gl::FALSE as GLint;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    }
    if success != gl::TRUE as GLint {
        let mut len = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        }
        let mut buffer = Vec::with_capacity(len as usize);
        buffer.extend([b' '].iter().cycle().take(len as usize));
        unsafe {
            gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
        }
        panic!("Erreur de compilation de shader: {}", String::from_utf8_lossy(&buffer));
    }

    shader
}
