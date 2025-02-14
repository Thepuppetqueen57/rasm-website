use macroquad::prelude::*;

fn get_text_length(text: &str, font_size: f32) -> f32 {
    let text_params = TextParams {
        font_size: font_size as u16,
        ..Default::default()
    };
    let text_dimensions = measure_text(text, None, text_params.font_size as u16, 1.0);
    text_dimensions.width
}

#[macroquad::main("RASM")]
async fn main() {
    let mut scroll: i16 = 0;
    let mut debug_mode: bool = false;
    let max_scroll: i16 = -4000;

    loop {
        // Logic
        let scroll_check = mouse_wheel().1;

        if scroll_check > 0.0 && scroll < 0 {
            scroll += 50
        } else if scroll_check < 0.0 && scroll > max_scroll {
            scroll -= 50
        }

        if is_key_pressed(KeyCode::D) {
            debug_mode = true
        }

        // Rendering
        clear_background(BLACK);

        draw_text(
            "Welcome to RASM!",
            screen_width() / 2.0 - get_text_length("Welcome to RASM", 50.0) / 2.0,
            screen_height() / 2.0 + scroll as f32,
            50.0,
            WHITE
        );

        draw_text(
            "RASM is an ASM inspired Programming Language made in Rust!",
            screen_width() / 2.0 - get_text_length("RASM is an ASM inspired Programming Language made in Rust!", 50.0) / 2.0,
            screen_height() / 2.0 + 1000.0 + scroll as f32,
            50.0,
            WHITE
        );

        draw_text(
            "RASM's Syntax is Short and Concise!",
            screen_width() / 2.0 - get_text_length("RASM's Syntax is Short and Concise!", 50.0) / 2.0,
            screen_height() / 2.0 + 2000.0 + scroll as f32,
            50.0,
            WHITE
        );

        draw_text(
            "Although RASM is in a very early state!",
            screen_width() / 2.0 - get_text_length("Although RASM is in a Very Early state!", 50.0) / 2.0,
            screen_height() / 2.0 + 3000.0 + scroll as f32,
            50.0,
            WHITE
        );

        draw_text(
            "At the moment RASM can't really do anything but that will hopefully change!",
            screen_width() / 2.0 - get_text_length("At the moment RASM can't really do anything but that will hopefully change!", 50.0) / 2.0,
            screen_height() / 2.0 + 4000.0 + scroll as f32,
            50.0,
            WHITE
        );

        if debug_mode {
            draw_text(&format!("Scroll: {}", scroll), 0.0, 20.0, 30.0, LIME);
            draw_text(&format!("Max Scroll: {}", max_scroll), 0.0, 50.0, 30.0, LIME);
        }

        next_frame().await
    }
}