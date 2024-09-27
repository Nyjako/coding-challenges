use nannou::{color::RgbHue, prelude::*};
use math::round;


const WIDTH:  u32       = 800;
const HEIGHT: u32       = 800;
const CELL_WIDTH: u32   = 10;
const CELL_HEIGHT: u32  = 10;
const COLUMNS: usize    = (WIDTH / CELL_WIDTH) as usize;
const ROWS: usize       = (HEIGHT / CELL_HEIGHT) as usize;
const ARRAY_SIZE: usize = COLUMNS * ROWS;
const MOUSE_RADIUS: f32 = 2.0;

fn main() {
    nannou::app(model)
        .size(WIDTH, HEIGHT)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

struct Model {
    board: [f32; ARRAY_SIZE],
    current_cell: Vec2,
    mouse_radius: f32,
    hue_value: f32,
}

fn model(_app: &App) -> Model {
    Model {
        board: [0.0; ARRAY_SIZE],
        current_cell: Vec2::ZERO,
        mouse_radius: MOUSE_RADIUS,
        hue_value: 0.0,
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent {
        simple: Some(MouseWheel(delta, _)),
        ..
    } = event
    {
        match delta {
            MouseScrollDelta::LineDelta(_, y) => {
                model.mouse_radius += y;
            }
            MouseScrollDelta::PixelDelta(pos) => {
                model.mouse_radius += pos.y as f32;
            }
        }

        model.mouse_radius = f32::max(model.mouse_radius, 1.0);
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let button = app.mouse.buttons.left();

    let bound = app.window_rect();
    let mouse_pos = app.mouse.position() + pt2(bound.right(), bound.top());

    let current_cell = pt2(
        round::floor((mouse_pos.x / CELL_WIDTH as f32) as f64, 0) as f32,
        ROWS as f32 - 1.0 - round::floor((mouse_pos.y / CELL_HEIGHT as f32) as f64, 0) as f32 
        // I need to reverse this because I start drawing from bottom left
    );

    model.current_cell = current_cell; // Spawn sand
    if button.is_down() {
        let first_low_val = f32::max(current_cell.x - model.mouse_radius as f32, 0.0) as usize;
        let first_high_val = f32::min(current_cell.x + model.mouse_radius + 1 as f32, COLUMNS as f32) as usize;
        for i in first_low_val..first_high_val {
            let second_low_val = f32::max(current_cell.y - model.mouse_radius as f32, 0.0) as usize;
            let second_high_val = f32::min(current_cell.y + model.mouse_radius + 1 as f32, ROWS as f32) as usize;
            for j in second_low_val..second_high_val {
                if model.board[ i + j * ROWS ] == 0.0 && pt2(i as f32, j as f32).distance(current_cell) <= model.mouse_radius {

                    model.board[ i + j * ROWS ] = model.hue_value;
                    model.hue_value = (model.hue_value + 0.1) % 360.0;
                }
            }
        }
    }

    for i in (0..COLUMNS).rev() { // Move sand
        for j in (0..ROWS).rev() {
            if model.board[ i + j * ROWS ] > 0.0 && j < ROWS - 1 {
                // random direction for sand to fall first
                let dir: i32 = if rand::random() { 1 } else { -1 };
                let temp1_i = i32::max(i32::min(i as i32 + dir, COLUMNS as i32 - 1), 0) as usize;
                let temp2_i = i32::max(i32::min(i as i32 - dir, COLUMNS as i32 - 1), 0) as usize;

                if model.board[ i + (j + 1) * ROWS ] == 0.0 {
                    model.board[ i + (j + 1) * ROWS ] = model.board[ i + j * ROWS ];
                    model.board[ i + j * ROWS ] = 0.0;
                }
                else if model.board[ temp1_i + (j + 1) * ROWS ] == 0.0 {
                    model.board[ temp1_i + (j + 1) * ROWS ] = model.board[ i + j * ROWS ];
                    model.board[ i + j * ROWS ] = 0.0;
                }
                else if model.board[ temp2_i + (j + 1) * ROWS ] == 0.0 {
                    model.board[ temp2_i + (j + 1) * ROWS ] = model.board[ i + j * ROWS ];
                    model.board[ i + j * ROWS ] = 0.0;
                }
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    frame.clear(Rgb::new(51 as u8, 51 as u8, 51 as u8));

    let boundary = app.window_rect();
    let draw = app.draw().x_y(boundary.left(), boundary.bottom()); // (0,0) is now left bottom

    draw_grid(&draw, model);

    draw.to_frame(app, &frame).unwrap();
}


fn draw_grid(draw: &Draw, model: &Model) {
    for i in 0..COLUMNS {
        let x: u32 = (i as u32) * CELL_WIDTH;
        
        for j in 0..ROWS {
            let y: u32 = (j as u32) * CELL_HEIGHT;

            if pt2(i as f32, j as f32).distance(model.current_cell) <= model.mouse_radius { // display mouse position
                draw.rect()
                    .xy(pt2((x + CELL_WIDTH / 2) as f32, (HEIGHT - y - CELL_HEIGHT / 2) as f32))
                    .w_h(CELL_WIDTH as f32, CELL_HEIGHT as f32)
                    // .stroke_weight(3.0).stroke(BLACK)
                    .color(PURPLE);
            }
            else if model.board[ i + j * ROWS ] > 0.0 { // Display sand
                draw.rect()
                    .xy(pt2((x + CELL_WIDTH / 2) as f32, (HEIGHT - y - CELL_HEIGHT / 2) as f32))
                    .w_h(CELL_WIDTH as f32, CELL_HEIGHT as f32)
                    // .stroke_weight(3.0).stroke(BLACK)
                    .color(Hsl::new(RgbHue::from_degrees(model.board[ i + j * ROWS ]), 0.5, 0.5));
            }
        }
    }
}