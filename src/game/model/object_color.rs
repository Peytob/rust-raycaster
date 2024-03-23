use sdl2::pixels::Color;

#[derive(Copy, Clone)]
pub enum ObjectColor {
    COLOR {
        color: &'static Color
    },

    TEXTURE {

    }
}

impl ObjectColor {

    pub const BLACK: ObjectColor = ObjectColor::COLOR {
        color: &Color::BLACK
    };

    pub const GRAY: ObjectColor = ObjectColor::COLOR {
        color: &Color::GRAY
    };

    pub const RED: ObjectColor = ObjectColor::COLOR {
        color: &Color::RED
    };

    pub const GREEN: ObjectColor = ObjectColor::COLOR {
        color: &Color::GREEN
    };

    pub const BLUE: ObjectColor = ObjectColor::COLOR {
        color: &Color::BLUE
    };

    pub const MAGENTA: ObjectColor = ObjectColor::COLOR {
        color: &Color::MAGENTA
    };

    pub const CYAN: ObjectColor = ObjectColor::COLOR {
        color: &Color::CYAN
    };

    pub const YELLOW: ObjectColor = ObjectColor::COLOR {
        color: &Color::YELLOW
    };

    pub const WHITE: ObjectColor = ObjectColor::COLOR {
        color: &Color::WHITE
    };
}