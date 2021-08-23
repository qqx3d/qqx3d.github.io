#[qqx::qqx(polygon)]
struct Vertex {
    pos: qqx::Vec2 <f32>,
    color: qqx::Color
}

fn main() {
    let window = qqx::Window::new().build();

    let triangle = qqx::Polygon::new().vertex(Vertex::new()
        .pos(qqx::Vec2::new(-0.5, -0.5))
        .color(qqx::Color::RED))
    .vertex(Vertex::new()
        .pos(qqx::Vec2::new(0.0, 0.5))
        .color(qqx::Color::GREEN))
    .vertex(Vertex::new()
        .pos(qqx::Vec2::new(0.5, -0.25))
        .color(qqx::Color::BLUE)
    ).bind(window);

    qqx::callback::on_frame(move || {
        window.draw()
            .clear(qqx::Color::from(qqx::Vec4::new(0.0, 0.1, 1.0, 1.0)))
            .draw(&triangle)
            .finish();

        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        return qqx::ControlFlow::WaitUntil(next_frame_time);
    });

    qqx::eventloop();
}
