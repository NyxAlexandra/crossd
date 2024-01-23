use crossd_math::Point2;
use crossd_scene::{Fill, Image, Path, Stroke};

pub trait Renderer {
    fn stroke(&mut self, path: Path, stroke: Stroke);

    fn fill(&mut self, path: Path, fill: Fill);

    fn image(&mut self, point: Point2, image: Image);
}

#[cfg(feature = "wgpu")]
mod crossd_graphics_impl {
    use crossd_graphics::graphics::Graphics;
    use crossd_graphics::scene::{Fill, Image, Path, Stroke};
    use crossd_math::Point2;

    use super::Renderer;

    impl Renderer for Graphics {
        fn stroke(&mut self, path: Path, stroke: Stroke) {
            todo!()
        }

        fn fill(&mut self, path: Path, fill: Fill) {
            todo!()
        }

        fn image(&mut self, point: Point2, image: Image) {
            todo!()
        }
    }
}

#[cfg(feature = "tiny-skia")]
mod tiny_skia_impl {
    use crossd_math::Point2;
    use crossd_scene::{
        Blend,
        Cap,
        Color,
        Fill,
        Image,
        Join,
        Path,
        PathVerb,
        Source,
        Stroke,
    };
    use tiny_skia::Pixmap;

    use super::Renderer;

    fn skia_path(path: impl Into<Path>) -> tiny_skia::Path {
        let path = path.into();
        let mut builder = tiny_skia::PathBuilder::with_capacity(
            path.verbs().len(),
            path.points().len(),
        );

        let mut verbs = path.verbs().iter();
        let mut points = path.points().iter();

        let mut next = || *points.next().unwrap();

        while let Some(verb) = verbs.next() {
            match verb {
                PathVerb::Move => {
                    let Point2 { x, y } = next();

                    builder.move_to(x, y);
                },
                PathVerb::Line => {
                    let Point2 { x, y } = next();

                    builder.line_to(x, y);
                },
                PathVerb::Quad => {
                    let Point2 { x, y } = next();

                    let c = next();

                    builder.quad_to(c.x, c.y, x, y);
                },
                PathVerb::Cubic => {
                    let Point2 { x, y } = next();

                    let c1 = next();
                    let c2 = next();

                    builder.cubic_to(c1.x, c1.y, c2.x, c2.y, x, y);
                },
                PathVerb::Close => builder.close(),
            }
        }

        builder.finish().unwrap()
    }

    fn skia_stroke(stroke: Stroke) -> (tiny_skia::Paint<'static>, tiny_skia::Stroke) {
        let paint = tiny_skia::Paint {
            shader: match stroke.source {
                Source::Solid(Color { r, g, b, a }) => tiny_skia::Shader::SolidColor(
                    tiny_skia::Color::from_rgba(r, g, b, a).unwrap(),
                ),
                Source::Gradient(_) => todo!(),
            },
            blend_mode: match stroke.blend {
                Blend::Src => tiny_skia::BlendMode::Source,
                Blend::Dst => tiny_skia::BlendMode::Destination,
                Blend::SrcOver => tiny_skia::BlendMode::SourceOver,
                Blend::DstOver => tiny_skia::BlendMode::DestinationOver,
            },
            ..Default::default()
        };
        let stroke = tiny_skia::Stroke {
            width: stroke.width,
            miter_limit: if let Join::Miter { limit } = stroke.join {
                limit
            } else {
                // `tiny_skia` default
                4.0
            },
            line_cap: match stroke.cap {
                Cap::Butt => tiny_skia::LineCap::Butt,
                Cap::Round => tiny_skia::LineCap::Round,
                Cap::Square => tiny_skia::LineCap::Square,
            },
            line_join: match stroke.join {
                Join::Round => tiny_skia::LineJoin::Round,
                Join::Bevel => tiny_skia::LineJoin::Bevel,
                Join::Miter { .. } => tiny_skia::LineJoin::Miter,
            },
            ..Default::default()
        };

        (paint, stroke)
    }

    fn skia_fill(fill: Fill) -> (tiny_skia::Paint<'static>, tiny_skia::FillRule) {
        todo!()
    }

    fn skia_pixmap(
        image: &Image,
    ) -> (tiny_skia::PixmapRef<'_>, tiny_skia::PixmapPaint, tiny_skia::Transform) {
        let pixmap =
            tiny_skia::PixmapRef::from_bytes(&image.bytes, image.size.w, image.size.h)
                .unwrap();
        let paint = tiny_skia::PixmapPaint::default();
        let transform = {
            let trans = image.trans;

            tiny_skia::Transform {
                sx: todo!(),
                kx: todo!(),
                ky: todo!(),
                sy: todo!(),
                tx: todo!(),
                ty: todo!(),
            }
        };

        (pixmap, paint, transform)
    }

    impl Renderer for Pixmap {
        fn stroke(&mut self, path: Path, stroke: Stroke) {
            let path = skia_path(path);
            let (paint, stroke) = skia_stroke(stroke);
            let transform = tiny_skia::Transform::identity();

            self.stroke_path(&path, &paint, &stroke, transform, None);
        }

        fn fill(&mut self, path: Path, fill: Fill) {
            let path = skia_path(path);
            let (paint, fill_rule) = skia_fill(fill);
            let transform = tiny_skia::Transform::identity();

            self.fill_path(&path, &paint, fill_rule, transform, None);
        }

        fn image(&mut self, point: Point2, image: Image) {
            let (pixmap, paint, transform) = skia_pixmap(&image);

            self.draw_pixmap(point.x as _, point.y as _, pixmap, &paint, transform, None);
        }
    }
}
