use super::{Backend, Color};
use widget::{WidgetCommon, WidgetManager, Widget};

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
