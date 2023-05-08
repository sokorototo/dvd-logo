use macroquad::prelude as mqd;

pub(crate) fn window_config() -> mqd::Conf {
    mqd::Conf {
        platform: Default::default(),
        window_title: String::from("DVD"),
        window_width: 800,
        window_height: 800,
        high_dpi: true,
        fullscreen: false,
        sample_count: 8,
        window_resizable: false,
        icon: None,
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let dvd_logo_data = include_bytes!("../dvd.png");
    let dvd_logo =
        mqd::Texture2D::from_file_with_format(dvd_logo_data, Some(mqd::ImageFormat::Png));
    dvd_logo.set_filter(mqd::FilterMode::Nearest);

    let mut color_counter = 0;
    let mut spamming = false;

    let scale = 0.1;
    let screen_dimensions = (mqd::screen_width(), mqd::screen_height());
    let texture_dimensions = (dvd_logo.width() * scale, dvd_logo.height() * scale);

    let mut texture_position = (0f32, 0f32);
    let mut texture_velocity = (2.25, 2f32);

    // Hide mouse
    mqd::show_mouse(false);

    loop {
        // Spasm the logo
        if mqd::is_key_pressed(mqd::KeyCode::Space) {
            spamming = !spamming;
        }

        // SPASM
        if spamming {
            color_counter += 1;
            color_counter = color_counter % 5;
        }

        // Update logic
        texture_position.0 += texture_velocity.0;
        texture_position.1 += texture_velocity.1;

        if texture_position.0 + texture_dimensions.0 > screen_dimensions.0
            || texture_position.0 < 0.
        {
            texture_velocity.0 = -texture_velocity.0;
            color_counter += 1;
        }

        if texture_position.1 + texture_dimensions.1 > screen_dimensions.1
            || texture_position.1 < 0.
        {
            texture_velocity.1 = -texture_velocity.1;
            color_counter += 1;
        }

        // Draw
        let color = match color_counter {
            0 => mqd::RED,
            1 => mqd::GREEN,
            2 => mqd::WHITE,
            3 => mqd::BLUE,
            4 => mqd::DARKPURPLE,
            _ => {
                color_counter = 0;
                mqd::BROWN
            }
        };

        mqd::draw_texture_ex(
            dvd_logo,
            texture_position.0,
            texture_position.1,
            color,
            mqd::DrawTextureParams {
                dest_size: Some(mqd::Vec2::from(texture_dimensions)),
                source: None,
                rotation: 0f32,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );

        // Quit from application if escape is pressed
        if mqd::is_key_down(mqd::KeyCode::Escape) {
            break;
        };

        mqd::next_frame().await;
    }
}
