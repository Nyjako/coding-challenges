use nannou::prelude::*;
use nannou::wgpu::{CommandEncoder, TextureBuilder, TextureUsages};
use rayon::prelude::*;
use std::mem;

const WIDTH:  u32 = 400;
const HEIGHT: u32 = 400;

// Some varibles for formula
const DIFFUSION_RATE_A: f32 = 1.0;
const DIFFUSION_RATE_B: f32 = 0.5;
const FEED: f32             = 0.055;
const KILL: f32             = 0.062;

fn main() {
    nannou::app(model)
        .size(WIDTH, HEIGHT)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}


fn get_xy(i: usize) -> (usize, usize) {
    let x = i % WIDTH as usize;
    let y = i / WIDTH as usize;
    (x, y)
}

fn get_i(x: usize, y: usize) -> usize {
    let i = x + y * WIDTH as usize;
    i
}

#[derive(Debug, Clone, Copy)]
struct Chemical {
    a: f32,
    b: f32,
}

struct Model {
    grid: Vec<Chemical>,
    next_grid: Vec<Chemical>,
    texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    let main_window = app.main_window();
    let device = main_window.device();
    let texture = TextureBuilder::new()
        .size([WIDTH, HEIGHT])
        .format(wgpu::TextureFormat::Rgba8UnormSrgb)
        .usage(TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT)
        .build(device);

    let mut grid = vec![Chemical{a: 1.0, b: 0.0}; HEIGHT as usize * WIDTH as usize];

    for i in 150..250 {
        for j in 150..250 {
            grid[get_i(i, j)].b = 1.0
        }
    }

    Model {
        grid: grid.clone(),
        next_grid: grid,
        texture,
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {

}

fn laplace(grid: &Vec<Chemical>, center: usize) -> (f32, f32) {
    let (x, y) = get_xy(center);

    // Handle edge cases for grid boundaries
    if x == 0 || y == 0 || x == WIDTH as usize - 1 || y == HEIGHT as usize - 1 {
        return (grid[center].a, grid[center].b);
    }

    let mut sum_a: f32 = 0.0;
    let mut sum_b: f32 = 0.0;

    let weights = [
        (-1.0,  0,  0),  // center
        ( 0.2, -1,  0),  // left
        ( 0.2,  1,  0),  // right
        ( 0.2,  0,  1),  // below
        ( 0.2,  0, -1),  // above
        ( 0.05, -1, -1), // top-left
        ( 0.05, -1,  1), // bottom-left
        ( 0.05,  1, -1), // top-right
        ( 0.05,  1,  1), // bottom-right
    ];

    for (weight, dx, dy) in &weights {
        let neighbor_x = (x as isize + dx) as usize;
        let neighbor_y = (y as isize + dy) as usize;
        let neighbor_idx = get_i(neighbor_x, neighbor_y);

        sum_a += grid[neighbor_idx].a * weight;
        sum_b += grid[neighbor_idx].b * weight;
    }

    (sum_a, sum_b)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    mem::swap(&mut model.grid, &mut model.next_grid);

    model.next_grid
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, cell)| {
            let a = model.grid[i].a;
            let b = model.grid[i].b;
            let (laplace_a, laplace_b) = laplace(&model.grid, i);

            cell.a = (a + (DIFFUSION_RATE_A * laplace_a) - (a * (b * b)) + (FEED * (1.0 - a))).clamp(0.0, 1.0);
            cell.b = (b + (DIFFUSION_RATE_B * laplace_b) + (a * (b * b)) - ((KILL + FEED) * b)).clamp(0.0, 1.0);
        });
}


fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);
    let draw = app.draw();

    draw_grid(app, &model.texture, &model.grid, &draw);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_grid(app: &App, texture: &wgpu::Texture, grid: &Vec<Chemical>, draw: &Draw) {
    let texture = texture;
    let window = app.main_window();
    let device = window.device();
    
    let mut encoder: CommandEncoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("texture_upload"),
    });

    let mut pixels = vec![0u8; WIDTH as usize * HEIGHT as usize * 4];

    for (i, pixel) in pixels.chunks_mut(4).enumerate() {

        let a = grid[i].a;
        let b = grid[i].b;
        let c = (a - b) * 255.0;

        let intensity = c.clamp(0.0, 255.0) as u8;

        pixel[0] = intensity; 
        pixel[1] = intensity; 
        pixel[2] = intensity;
        pixel[3] = 255;  // Alpha (fully opaque)
    }

    texture.upload_data(device, &mut encoder, &pixels);
    window.queue().submit(Some(encoder.finish()));

    draw.texture(texture).w_h(WIDTH as f32, HEIGHT as f32);
}