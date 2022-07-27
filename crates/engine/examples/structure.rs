use firecore_engine::{
    events::{EventProcessor, WindowEvents},
    graphics::Graphics,
};
use winit::{
    event_loop::EventLoop,
    window::{Icon, WindowBuilder},
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

    event_loop.run(move |event, b, c| {
        if let Ok(Some(event)) = events.process(event, c, &window, &mut graphics) {
            match event {
                WindowEvents::Draw(should_update) => {
                    if let Some(delta) = should_update {
                        update(delta.as_secs_f32(), &mut red);
                    }
                    draw(red, &mut graphics);
                }
                WindowEvents::Exit => {}
            }
        }
    });
}

fn update(delta: f32, red: &mut f32) {
    println!("{delta}");
    *red += delta;
    *red %= 1.0;
}

fn draw(red: f32, graphics: &mut Graphics) {

    let mut output = graphics.create_view().unwrap();

    output.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[
            Some(wgpu::RenderPassColorAttachment {
                view: &output.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: red as _,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })
        ],
        depth_stencil_attachment: None,
    });

    output.finish(&graphics.queue);
}
