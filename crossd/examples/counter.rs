fn main() {}

// use crossd::signal::Signal;
// use crossd::widget::{Button, Column, Text};

// fn main() {
//     crossd::launch(|| {
//         let count = Signal::new(0);

//         Column::new((
//             Button::text((|| "+")).on_press(move || count.update(|count|
// count += 1)),             Text::new(|| format!("count: {}", count.get())),
//             Button::text(|| "-").on_press(move || count.update(|count| count
// -= 1)),         ))
//     })
// }
