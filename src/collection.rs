use super::Backend;
use widget::{Widget, WidgetCommon, WidgetManager};

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
