use super::{Backend, MouseEvent};
use collection::Collection;

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
