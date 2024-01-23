pub mod graphics;
pub mod layout;
pub mod signal;
pub mod view;
pub mod widget;

#[cfg(all(test, not(test)))]
fn example() -> crossd::Result {
    crossd::launch(|| {
        let count = Signal::new(0);

        column((
            text(move || format!("count: {}", count.get())),
            row((
                button(|| "+").on_click(move || count.update(|count| *count += 1)),
                button(|| "-").on_click(move || count.update(|count| *count -= 1)),
            )),
        ))
    })
}
