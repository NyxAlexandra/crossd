use crossd_scene::Scene;

pub trait Backend {
    fn render(&mut self, scene: &Scene);
}
