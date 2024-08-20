#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{
    display::{
        object::{Graphics, OamManaged, Tag},
        tiled::{RegularBackgroundSize, TileFormat, TiledMap},
        Font, Priority,
    },
    include_aseprite, include_font,
};
mod scenes;

static GRAPHICS: &Graphics = include_aseprite!("resources/gfx/sprites.aseprite");
static FONT: Font = include_font!("resources/font/font.ttf", 28);

static BALL: &Tag = GRAPHICS.tags().get("Ball");

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let (gfx, mut vram) = gba.display.video.tiled0();
    let mut input = agb::input::ButtonController::new();
    let vblank = agb::interrupt::VBlank::get();
    let object: OamManaged = gba.display.object.get_managed();
    let mut ball = object.object_sprite(BALL.sprite(0));

    vram.set_background_palette_raw(&[
        0x0000, 0x0ff0, 0x00ff, 0xf00f, 0xf0f0, 0x0f0f, 0xffff, 0x5555, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
    ]);

    let mut bg = gfx.background(
        Priority::P0,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    let mut manager = scenes::SceneManager::new();

    loop {
        let background_tile = vram.new_dynamic_tile().fill_with(6);
        for y in 0..20u16 {
            for x in 0..30u16 {
                bg.set_tile(
                    &mut vram,
                    (x, y),
                    &background_tile.tile_set(),
                    background_tile.tile_setting(),
                );
            }
        }
        let mut renderer = FONT.render_text((1u16, 3u16));
        let writer = renderer.writer(1, 6, &mut bg, &mut vram);
        manager.process(&input);

        vblank.wait_for_vblank();
        manager.render(&mut ball, &object, writer);
        bg.commit(&mut vram);
        bg.set_visible(true);
        object.commit();
        input.update();

        manager.clear(&mut ball);
        renderer.clear(&mut vram);
        bg.clear(&mut vram);
        vram.remove_dynamic_tile(background_tile);
    }
}
