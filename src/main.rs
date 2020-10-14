use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    Model {}
}

// runs every time some sort of app event occurs
fn event(_app: &App, _model: &mut Model, _event: Event) {}

// defines the view
fn view(app: &App, _model: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(RED);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

// could be used to run code on timed updates
fn _update(_app: &App, _model: &mut Model, _update: Update) {}
