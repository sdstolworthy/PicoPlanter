#![macro_use]
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Baseline, Text},
};
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, Ssd1306};

pub struct OledScreen<'a, DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: ssd1306::prelude::DisplaySize,
{
    line_height: i32,
    display: &'a mut Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>,
    text_style: MonoTextStyle<'a, BinaryColor>,
    clear_style: PrimitiveStyle<BinaryColor>,
}

impl<'a, DI, SIZE> OledScreen<'a, DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: ssd1306::prelude::DisplaySize,
{
    pub fn new(display: &'a mut Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>) -> Self {
        let clear_style = PrimitiveStyle::with_fill(BinaryColor::Off);
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();
        OledScreen {
            line_height: 12,
            display,
            text_style,
            clear_style,
        }
    }

    fn get_line_bound(&mut self, line: i32) -> Point {
        Point::new(0, line * self.line_height)
    }
    pub fn write(&mut self, s: &str, line: i32) -> () {
        let line_start = self.get_line_bound(line);
        self.clear_line(line);
        self.display.flush().unwrap();
        Text::with_baseline(s, line_start, self.text_style, Baseline::Top)
            .draw(self.display)
            .unwrap();
        self.update();
    }
    pub fn clear_line(&mut self, line: i32) -> () {
        let width = self.display.size().width;
        Rectangle::new(
            self.get_line_bound(line),
            Size::new(width, self.line_height as u32),
        )
        .into_styled(self.clear_style)
        .draw(self.display)
        .unwrap();
    }
    fn update(&mut self) -> () {
        self.display.flush().unwrap();
    }
}
