#[link(name = "gooey",
       vers = "0.1",
       uuid = "d5fe2f28-2ec7-415c-840c-16ef376f058f",
       author = "Josh Matthews",
       url = "https://github.com/jdm/gooey")];

#[comment = "A GUI toolkit abstraction."];
#[crate_type = "lib"];

struct WidgetCommon {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    id: u32
}

impl WidgetCommon {
    pub fn new(manager: &mut WidgetManager) -> WidgetCommon {
        WidgetCommon::new_with_id(manager.next_id())
    }

    pub fn new_with_id(id: u32) -> WidgetCommon {
        WidgetCommon {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
            id: id
        }
    }
}

pub trait Widget {
    fn paint(&self, backend: &Backend, off_x: u32, off_y: u32);
    fn common<'a>(&'a self) -> &'a WidgetCommon;

    fn x(&self) -> u32 { self.common().x }
    fn y(&self) -> u32 { self.common().y }
    fn width(&self) -> u32 { self.common().w }
    fn height(&self) -> u32 { self.common().h }
    fn id(&self) -> u32 { self.common().id }
}

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

pub struct WidgetManager {
    widgets: Collection,
    focused: Option<@Widget>,
    mouse_handlers: ~[~fn(ev: MouseEvent)],
    key_handlers: ~[~fn(key: u8)],
    next_widget_id: u32
}

impl WidgetManager {
    pub fn new() -> WidgetManager {
        WidgetManager {
            widgets: Collection {
                common: WidgetCommon::new_with_id(0),
                elements: ~[]
            },
            focused: None,
            mouse_handlers: ~[],
            key_handlers: ~[],
            next_widget_id: 1
        }
    }

    pub fn next_id(&mut self) -> u32 {
        let id = self.next_widget_id;
        self.next_widget_id += 1;
        id
    }

    pub fn add<T: 'static+Widget>(&mut self, widget: @mut T) {
        self.widgets.add(widget);
    }

    pub fn remove<T: Widget>(&mut self, widget: @mut T) {
        self.widgets.remove(widget);
    }

    pub fn focus(&mut self, widget: Option<@Widget>) {
        self.focused = widget
    }

    pub fn focused(&self) -> Option<@Widget> {
        self.focused
    }

    pub fn paint<T: Backend>(&self, backend: &T) {
        self.widgets.paint(backend as &Backend, 0, 0)
    }
}

pub struct Collection {
    common: WidgetCommon,
    elements: ~[@mut Widget],
}

impl Collection {
    pub fn new(manager: &mut WidgetManager) -> @mut Collection {
        @mut Collection {
            common: WidgetCommon::new(manager),
            elements: ~[],
        }
    }
    
    pub fn add<T: 'static+Widget>(&mut self, widget: @mut T) {
        self.elements.push(widget as @mut Widget);
        let bounding_w = widget.x() + widget.width();
        if bounding_w > self.common.w {
            self.common.w = bounding_w;
        }
        let bounding_h = widget.y() + widget.height();
        if bounding_h > self.common.w {
            self.common.h = bounding_h;
        }
    }

    pub fn remove<T: Widget>(&mut self, widget: @mut T) {
        let mut index = None;
        for (pos, elem) in self.elements.iter().enumerate() {
            if elem.id() == widget.id() {
                index = Some(pos);
            }
        }

        for &pos in index.iter() {
            self.elements.remove(pos);

            let mut bounding_w = 0;
            let mut bounding_h = 0;
            for elem in self.elements.iter() {
                let cur_bounding_w = elem.x() + elem.width();
                let cur_bounding_h = elem.y() + elem.height();
                if cur_bounding_w > bounding_w {
                    bounding_w = cur_bounding_w;
                }
                if cur_bounding_h > bounding_h {
                    bounding_h = cur_bounding_h;
                }
            }
            self.common.w = bounding_w;
            self.common.h = bounding_h;
        }
    }
}

impl Widget for Collection {
    fn paint(&self, backend: &Backend, off_x: u32, off_y: u32) {
        for elem in self.elements.iter() {
            elem.paint(backend, self.common.x + off_x, self.common.y + off_y);
        }
    }

    fn common<'a>(&'a self)  -> &'a WidgetCommon {
        &self.common
    }
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

pub struct Border {
    top_thickness: u32,
    bottom_thickness: u32,
    left_thickness: u32,
    right_thickness: u32,

    top_color: Color,
    bottom_color: Color,
    left_color: Color,
    right_color: Color
}

impl Border {
    pub fn new_all_same(thickness: u32, color: Color) -> Border{
        Border {
            top_thickness: thickness,
            bottom_thickness: thickness,
            left_thickness: thickness,
            right_thickness: thickness,

            top_color: color,
            bottom_color: color,
            left_color: color,
            right_color: color
        }
    }

    pub fn new_dual_color(thickness: u32, upper: Color, lower: Color) -> Border {
        Border {
            top_thickness: thickness,
            bottom_thickness: thickness,
            left_thickness: thickness,
            right_thickness: thickness,

            top_color: upper,
            left_color: upper,
            bottom_color: lower,
            right_color: lower
        }
    }
}

pub struct Box {
    common: WidgetCommon,
    border: Border,
    background: Color
}

impl Box {
    pub fn new(manager: &mut WidgetManager, x: u32, y: u32, w: u32, h: u32,
               border: Border, background: Color) -> @mut Box {
        let box = @mut Box {
            common: WidgetCommon::new(manager),
            border: border,
            background: background
        };
        box.common.x = x;
        box.common.y = y;
        box.common.w = w;
        box.common.h = h;
        box
    }
}

impl Widget for Box {
    fn common<'a>(&'a self) -> &'a WidgetCommon {
        &self.common
    }

    fn paint(&self, backend: &Backend, off_x: u32, off_y: u32) {
        let top_left = (self.common.x + off_x,
                        self.common.y + off_y);
        backend.fill_rect(top_left.first(),
                          top_left.second(),
                          self.common.w,
                          self.common.h,
                          self.background);
        backend.fill_rect(top_left.first(),
                          top_left.second(),
                          self.common.w,
                          self.border.top_thickness,
                          self.border.top_color);
        backend.fill_rect(top_left.first(),
                          top_left.second(),
                          self.border.left_thickness,
                          self.common.h,
                          self.border.left_color);
        backend.fill_rect(top_left.first(),
                          top_left.second() + self.common.h - self.border.bottom_thickness,
                          self.common.w,
                          self.border.bottom_thickness,
                          self.border.bottom_color);
        backend.fill_rect(top_left.first() + self.common.w - self.border.right_thickness,
                          top_left.second(),
                          self.border.right_thickness,
                          self.common.h,
                          self.border.right_color);
    }
}

pub trait Animation {
    fn advance(&mut self);
    fn is_finished(&self);
}
