extern mod sdl;
extern mod gooey;

use std::libc;

use gooey::animation::AnimationManager;
use gooey::widget::WidgetManager;
use gooey::box::{Border, Box};

#[no_mangle]
pub extern fn SDL_main(_argc: u32, _argv: **libc::c_char) {
    main();
}

pub struct SDLBackend<'self> {
    screen: &'self sdl::video::Surface
}

impl<'self> gooey::Backend for SDLBackend<'self> {
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

    let mut animations = AnimationManager::new();

    let mut manager = WidgetManager::new();
    let border = Border::new_dual_color(4,
                                        gooey::Color::from_rgb(0x99999900),
                                        gooey::Color::from_rgb(0x77777700));
    let background = gooey::Color::from_rgb(0x88888800);
    let eventual_y = 20;
    let eventual_h = 500;
    let box = Box::new(&mut manager, 10, eventual_y + eventual_h / 2,
                       700, 0, border, background);
    manager.add(box);

    animations.add(|| {
        let increment = 25;
        box.common.h += increment;
        box.common.y -= increment / 2;
        if box.common.h < eventual_h {
            Some(10)
        } else {
            None
        }
    }, 10);

    'main : loop {
        animations.run();

        screen.fill(sdl::video::RGB(0, 0, 0));
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
