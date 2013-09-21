extern mod sdl;
extern mod gooey;

use std::libc;

#[no_mangle]
pub extern fn SDL_main(_argc: u32, _argv: **libc::c_char) {
    main();
}

pub struct SDLBackend<'self> {
    screen: &'self sdl::video::Surface
}

impl<'self> gooey::Backend for SDLBackend<'self> {
    fn draw_horiz_line(&self, x: u32, y: u32, w: u32, color: gooey::Color) {
        do self.screen.with_lock |pixels| {
            for i in range(x, x + w) {
                unsafe {
                    let idx = (*self.screen.raw).pitch as u32 * y + i * 4; //XXXjdm hardcoded bpp
                    pixels[idx] = color.r;
                    pixels[idx + 1] = color.g;
                    pixels[idx + 2] = color.b;
                }
            }
        }
    }

    fn draw_vert_line(&self, x: u32, y: u32, h: u32, color: gooey::Color) {
        do self.screen.with_lock |pixels| {
            for i in range(0, h) {
                unsafe {
                let idx = (*self.screen.raw).pitch as u32 * (y + i)  + x * 4; //XXXjdm hardcode bpp
                    pixels[idx] = color.r;
                    pixels[idx + 1] = color.g;
                    pixels[idx + 2] = color.b;
                }
            }
        }
    }

    fn fill_rect(&self, x: u32, y: u32, w: u32, h: u32, color: gooey::Color) {
        self.screen.fill_rect(Some(sdl::Rect(x as i16, y as i16, w as u16, h as u16)),
                              sdl::video::RGB(color.r, color.g, color.b));
    }
}

#[main]
pub fn main() {
    sdl::init([sdl::InitVideo]);
    sdl::wm::set_caption("rust-sdl demo - video", "rust-sdl");
    let screen = match sdl::video::set_video_mode(800, 600, 32, [sdl::video::HWSurface],
                                                                [sdl::video::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => fail!(fmt!("failed to set video mode: %s", err))
    };

    let backend = SDLBackend {
        screen: screen
    };

    let mut manager = gooey::WidgetManager::new();
    let box = gooey::Box::new(&mut manager, 10, 20, 700, 500);
    manager.add(box);

    'main : loop {
        manager.paint(&backend);
        screen.flip();

        'event : loop {
            match sdl::event::poll_event() {
                sdl::event::QuitEvent => break 'main,
                sdl::event::NoEvent => break 'event,
                sdl::event::KeyEvent(k, _, _, _)
                    if k == sdl::event::EscapeKey
                        => break 'main,
                _ => {}
            }
        }
    }

    sdl::quit();
}
