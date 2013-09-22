#[link(name = "gooey",
       vers = "0.1",
       uuid = "d5fe2f28-2ec7-415c-840c-16ef376f058f",
       author = "Josh Matthews",
       url = "https://github.com/jdm/gooey")];

#[comment = "A GUI toolkit abstraction."];
#[crate_type = "lib"];

pub use self::collection::Collection;
pub use self::widget::Widget;

pub mod animation;
pub mod box;
pub mod collection;
pub mod widget;

pub enum MouseEvent {
    MouseOver { x: u32, y: u32 },
    MouseDown { x: u32, y: u32, button: u8 },
    MouseUp { x: u32, y: u32, button: u8 },
    Click { x: u32, y: u32, button: u8 },
    DoubleClick { x: u32, y: u32, button: u8 },
    WheelDown { x: u32, y: u32, amount: u8 },
    WheelUp { x: u32, y: u32, amount: u8 }
}

pub trait Backend {
    fn draw_horiz_line(&self, x: u32, y: u32, w: u32, color: Color);
    fn draw_vert_line(&self, x: u32, y: u32, h: u32, color: Color);
    fn fill_rect(&self, x: u32, y: u32, w: u32, h: u32, color: Color);
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Color {
    pub fn from_components(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: 255
        }
    }

    pub fn from_rgba(rgba: u32) -> Color {
        Color {
            r: ((rgba & 0xFF000000) >> 24) as u8,
            g: ((rgba & 0x00FF0000) >> 16) as u8,
            b: ((rgba & 0x0000FF00) >> 8) as u8,
            a: ((rgba & 0x000000FF) >> 0) as u8
        }
    }

    pub fn from_rgb(rgb: u32) -> Color {
        Color::from_rgba(rgb & 0xFFFFFF00 | 0x000000FF)
    }
}
