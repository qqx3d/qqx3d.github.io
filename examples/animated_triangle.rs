#[qqx::qqx(polygon)]
struct Vertex {
    #[mutable]
    pos: qqx::Vec2 <f32>,
    color: qqx::Color
}

fn main() {
    let window = qqx::Window::new().build();

    let mut triangle = qqx::Polygon::new().vertex(Vertex::new()
        .pos(qqx::Vec2::new(-0.5, -0.5))
        .color(qqx::Color::RED))
    .vertex(Vertex::new()
        .pos(qqx::Vec2::new(0.0, 0.5))
        .color(qqx::Color::GREEN))
    .vertex(Vertex::new()
        .pos(qqx::Vec2::new(0.5, -0.25))
        .color(qqx::Color::BLUE)
    ).bind(window);

    triangle.set_pos(qqx::Vec2::new(-0.5, 0.0));
    qqx::callback::on_frame(move || {
        triangle.r#move(qqx::Vec2::new(0.0002, 0.0));
        if triangle.get_pos().x > 0.5 {
            triangle.set_pos(qqx::Vec2::new(-0.5, 0.0));
        }

        window.draw()
            .clear(qqx::Color::from(qqx::Vec4::new(0.0, 0.1, 1.0, 1.0)))
            .draw(&triangle)
            .finish();

        qqx::ControlFlow::Poll
    });

    qqx::eventloop();
}

