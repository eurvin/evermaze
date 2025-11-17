use nannou::prelude::*;
use rand::Rng;
use rand::distr::StandardUniform;
use rand::prelude::Distribution;

const ROWS: u32 = 30;
const COLS: u32 = 30;
const SIZE: u32 = 30;
const WIDTH: u32 = ROWS * SIZE;
const HEIGHT: u32 = COLS * SIZE;
const LINE_WIDTH: f32 = 0.05;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::refresh_sync())
        .run()
}

struct Model {
    rot_adj: f32,
    walls: Vec<Wall>,
}

#[derive(Debug)]
enum DiagonalLine {
    Forward,
    Backward,
}

impl Distribution<DiagonalLine> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DiagonalLine {
        let index: u8 = rng.random_range(0..2);
        match index {
            0 => DiagonalLine::Forward,
            1 => DiagonalLine::Backward,
            _ => unreachable!(),
        }
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();

    let rot_adj = 1.0;
    let mut walls = Vec::new();
    for x in 0..COLS {
        for y in 0..ROWS {
            let wall = Wall::new(x as f32, y as f32);
            walls.push(wall);
        }
    }
    Model { rot_adj, walls }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let gdraw = draw
        .scale(SIZE as f32)
        .scale_y(-1.0)
        .x_y(ROWS as f32 / -2.0, COLS as f32 / -2.0);

    draw.background().color(DARKSLATEBLUE);

    for wall in &model.walls {
        let cdraw = &gdraw;
        let start_point: Vec2;
        let end_point: Vec2;
        match wall.diagonal {
            DiagonalLine::Forward => {
                start_point = pt2(wall.x, wall.y + 1.0);
                end_point = pt2(wall.x + 1.0, wall.y);
            }
            DiagonalLine::Backward => {
                start_point = pt2(wall.x, wall.y);
                end_point = pt2(wall.x + 1.0, wall.y + 1.0)
            }
        }
        cdraw
            .line()
            .start(start_point)
            .end(end_point)
            .weight(LINE_WIDTH)
            .color(SNOW);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for wall in &mut model.walls {
        wall.rotation = rand::random_bool(0.00005);
        if wall.rotation {
            wall.diagonal = rand::random();
        }
    }
}

#[derive(Debug)]
struct Wall {
    diagonal: DiagonalLine,
    x: f32,
    y: f32,
    rotation: bool,
}

impl Wall {
    fn new(x: f32, y: f32) -> Self {
        let diagonal = rand::random();
        let rotation = false;
        Wall {
            diagonal,
            x,
            y,
            rotation,
        }
    }
}
