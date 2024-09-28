use nannou::prelude::*;

const WIDTH:  u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    nannou::app(model)
        .size(WIDTH, HEIGHT)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {

}

fn update(_app: &App, _model: &mut Model, _update: Update) {

}

fn view(_app: &App, _model: &Model, frame: Frame){
    frame.clear(PURPLE);
}
