use sdl2::video::Window;

pub fn init_sdl2_window() -> (sdl2::Sdl, Window) {
    // Initialiser SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // Configurer le profil OpenGL
    // Créer une fenêtre avec SDL2
    let window = video_subsystem
        .window("Fenêtre OpenGL avec SDL2", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    (sdl_context, window)
}
