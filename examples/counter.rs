use crossd::scene::{Rect, Scene};
use crossd_core::scene::{Color, Fill, FillRule, Point2, Size2, Source};

fn main() {
    let mut scene = Scene::new();

    scene.fill(
        Rect::new(Point2::ZERO, Size2::new(200.0, 100.0)),
        &Fill {
            source: Source::Color(Color::BLUE),
            rule: FillRule::EvenOdd,
            ..Default::default()
        },
    );
}

// use crossd::widget::column::Column;

// fn main() -> crossd::Result {
//     crossd::launch(0u32, |count| {
//         Column::new((
//             Text::new(format!("count: {count}")),
//             Row::new((
//                 Button::new("+").on_press(|data| data += 1),
//                 Button::new("-").on_press(|data| data -= 1),
//             )),
//         ))
//     })
// }
