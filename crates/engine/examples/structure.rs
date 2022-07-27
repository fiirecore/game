use firecore_engine::{
    events::{EventProcessor, WindowEvents},
    graphics::{draw::Draw, texture::Texture, Graphics},
};
use winit::{
    event_loop::EventLoop,
    window::{WindowBuilder},
};

fn main() {
    let assets = ();

    // assets.load::<Texture>("texture.png");

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Example")
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    let mut graphics = pollster::block_on(Graphics::new(&window)).unwrap();

    let mut events = EventProcessor::default();

    window.set_visible(true);

    let mut red = 0.0;

    let texture = Texture::from_bytes(
        &graphics,
        include_bytes!("../../../assets/battle/trainers/brock.png"),
    )
    .unwrap();

    event_loop.run(move |event, b, c| {
        if let Ok(Some(event)) = events.process(event, c, &window, &mut graphics) {
            match event {
                WindowEvents::Draw(should_update) => {
                    if let Some(delta) = should_update {
                        update(delta.as_secs_f32(), &mut red);
                    }
                    draw(red, &mut graphics, &texture);
                }
                WindowEvents::Exit => {}
            }
        }
    });
}

fn update(delta: f32, red: &mut f32) {
    *red += delta;
    *red %= 1.0;
}

fn draw(red: f32, graphics: &mut Graphics, texture: &Texture) {
    let mut output = graphics.create_view().unwrap();

    let pass = output.draw(
        Some("Pass"),
        Some(wgpu::Color {
            r: red as _,
            g: 0.2,
            b: 0.4,
            a: 1.0,
        }),
    );

    let mut draw = Draw(pass);

    draw.texture(texture, 5.0, 5.0);

    draw.finish();

    output.finish(&graphics.queue);
}
