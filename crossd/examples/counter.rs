fn main() {}

// use crossd::widget::{column, text, row, button}:
//
// fn main() -> crossd::Result {
// crossd::launch(|rt| {
// let (count, set_count) = rt.create_signals(0);
//
// column((
// text(|| format!("count: {}", count.get())),
// row((
// button(text(|| "+"))
// .on_press(move || set_count.update(|count| count += 1)),
// button(text(|| "-"))
// .on_press(move || set_count.update(|count| count -= 1)),
// )),
// ))
// })
// }
