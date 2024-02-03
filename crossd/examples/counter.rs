use crossd::signal::Signal;

fn main() -> crossd::Result {
    crossd::launch(|| {
        let count = Signal::new(0);
    })
}
