//Ball example from https://github.com/jagregory/abrash-black-book/blob/master/src/chapter-23.md
use std::sync::Arc;
use vga::screen;
use vga::{CRTReg, GCReg, SCReg};

const LOGICAL_SCREEN_WIDTH: usize = 672 / 8; //width in bytes and height in scan
const LOGICAL_SCREEN_HEIGHT: usize = 384; //lines of the virtual screen we'll work with
const PAGE0: usize = 0; //flag for page 0 when page flipping
const PAGE1: usize = 1; //flag for page 1 when page flipping
const PAGE0_OFFSET: usize = 0; //start offset of page 0 in VGA memory
const PAGE1_OFFSET: usize = LOGICAL_SCREEN_WIDTH * LOGICAL_SCREEN_HEIGHT; //start offset of page 1 (both pages are 672x384 virtual screens)
const BALL_WIDTH: usize = 24 / 8; //width of ball in display memory bytes
const BALL_HEIGHT: usize = 24; //height of ball in scan lines
const BLANK_OFFSET: usize = PAGE1_OFFSET * 2; //start of blank image in VGA memory
const BALL_OFFSET: usize = BLANK_OFFSET + (BALL_WIDTH * BALL_HEIGHT); //start offset of ball image in VGA memory
const NUM_BALLS: usize = 4;

pub fn main() {
    let mut vga = vga::new(0x10);

    draw_border(&mut vga, PAGE0_OFFSET);

    vga.set_sc_data(SCReg::MapMask, 0x01);
    vga.write_mem_chunk(
        BALL_OFFSET,
        &vec![
            0x00, 0x3c, 0x00, 0x01, 0xff, 0x80, //
            0x07, 0xff, 0xe0, 0x0f, 0xff, 0xf0, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x7f, 0xff, 0xfe, 0xff, 0xff, 0xff, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x7f, 0xff, 0xfe, 0x3f, 0xff, 0xfc, //
            0x3f, 0xff, 0xfc, 0x1f, 0xff, 0xf8, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        ],
    );
    vga.set_sc_data(SCReg::MapMask, 0x02);
    vga.write_mem_chunk(
        BALL_OFFSET,
        &vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x1f, 0xff, 0xf8, 0x3f, 0xff, 0xfc, //
            0x3f, 0xff, 0xfc, 0x7f, 0xff, 0xfe, //
            0x7f, 0xff, 0xfe, 0xff, 0xff, 0xff, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x0f, 0xff, 0xf0, 0x07, 0xff, 0xe0, //
            0x01, 0xff, 0x80, 0x00, 0x3c, 0x00, //
        ],
    );
    vga.set_sc_data(SCReg::MapMask, 0x04);
    vga.write_mem_chunk(
        BALL_OFFSET,
        &vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0xff, 0xff, 0xff, 0x7f, 0xff, 0xfe, //
            0x7f, 0xff, 0xfe, 0x3f, 0xff, 0xfc, //
            0x3f, 0xff, 0xfc, 0x1f, 0xff, 0xf8, //
            0x0f, 0xff, 0xf0, 0x07, 0xff, 0xe0, //
            0x01, 0xff, 0x80, 0x00, 0x3c, 0x00, //
        ],
    );
    vga.set_sc_data(SCReg::MapMask, 0x08);
    vga.write_mem_chunk(
        BALL_OFFSET,
        &vec![
            0x00, 0x3c, 0x00, 0x01, 0xff, 0x80, //
            0x07, 0xff, 0xe0, 0x0f, 0xff, 0xf0, //
            0x1f, 0xff, 0xf8, 0x3f, 0xff, 0xfc, //
            0x3f, 0xff, 0xfc, 0x7f, 0xff, 0xfe, //
            0x7f, 0xff, 0xfe, 0xff, 0xff, 0xff, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, //
            0xff, 0xff, 0xff, 0x7f, 0xff, 0xfe, //
            0x7f, 0xff, 0xfe, 0x3f, 0xff, 0xfc, //
            0x3f, 0xff, 0xfc, 0x1f, 0xff, 0xf8, //
            0x0f, 0xff, 0xf0, 0x07, 0xff, 0xe0, //
            0x01, 0xff, 0x80, 0x00, 0x3c, 0x00, //
        ],
    );
    vga.set_sc_data(SCReg::MapMask, 0x0F);
    for i in 0..(BALL_WIDTH * BALL_HEIGHT) {
        vga.write_mem(BLANK_OFFSET + i, 0x00);
    }

    //enable write mode 1
    let mut gc_mode = vga.get_gc_data(GCReg::GraphicsMode);
    gc_mode &= 0xFC;
    gc_mode |= 0x01;
    vga.set_gc_data(GCReg::GraphicsMode, gc_mode);

    //set scan line width (in bytes)
    vga.set_crt_data(CRTReg::Offset, (LOGICAL_SCREEN_WIDTH / 2) as u8);

    screen::start(Arc::new(vga));
}

fn draw_border(vga: &mut vga::VGA, offset: usize) {
    let mut di = offset;
    for _ in 0..(LOGICAL_SCREEN_HEIGHT / 16) {
        vga.set_sc_data(SCReg::MapMask, 0x0c); //red channel
        draw_border_block(vga, di);
        di += LOGICAL_SCREEN_WIDTH * 8;
        vga.set_sc_data(SCReg::MapMask, 0x0e); //yellow channel
        draw_border_block(vga, di);
        di += LOGICAL_SCREEN_WIDTH * 8;
    }
}

fn draw_border_block(vga: &mut vga::VGA, offset: usize) {
    let mut di = offset;
    for _ in 0..8 {
        vga.write_mem(di, 0xff);
        di += LOGICAL_SCREEN_WIDTH // - 1;
    }
}
