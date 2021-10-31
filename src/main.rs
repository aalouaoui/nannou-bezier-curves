// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 2-4: Forces Friction
use nannou::{prelude::*, state::mouse::ButtonPosition};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    points: [Vec2; 4],
    active_point: Option<usize>,
}

fn model(_app: &App) -> Model {
    let p0 = vec2(-50., -50.);
    let p1 = vec2(-50., 50.);
    let p2 = vec2(50., 50.);
    let p3 = vec2(50., -50.);
    Model {
        points: [p0, p1, p2, p3],
        active_point: None,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.active_point = match app.mouse.buttons.left() {
        ButtonPosition::Up => None,
        ButtonPosition::Down(pos) => match model.active_point {
            Some(index) => {
                model.points[index] = app.mouse.position();
                Some(index)
            }
            None => model
                .points
                .iter()
                .position(|point| point.distance(*pos) < 8.0),
        },
    };
}

fn calculate_point(&points: &[Vec2; 4], t: f32) -> Vec2 {
    let [p0, p1, p2, p3] = points;
    let t2 = t.powi(2);
    let t3 = t.powi(3);
    p0 * (-t3 + 3. * t2 - 3. * t + 1.)
        + p1 * (3. * t3 - 6. * t2 + 3. * t)
        + p2 * (-3. * t3 + 3. * t2)
        + p3 * t3
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let points = model.points;

    draw.line()
        .weight(1.0)
        .points(points[0], points[1])
        .color(GREY);
    draw.line()
        .weight(1.0)
        .points(points[2], points[3])
        .color(GREY);

    draw.polyline().weight(5.0).points_colored(
        (0..=100)
            .map(|t| t as f32 / 100.0)
            .map(|t| (calculate_point(&points, t), hsl(t / 2.0, 1.0, 0.5))),
    );

    points.iter().for_each(|&point| {
        draw.ellipse()
            .xy(point)
            .radius(7.)
            .stroke(WHITE)
            .stroke_weight(1.0)
            .color(BLACK);
    });

    draw.to_frame(app, &frame).unwrap();
}
