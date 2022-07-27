pub extern crate image;
pub extern crate wgpu;
pub extern crate winit;

use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

use self::texture::pipeline::DefaultTexturePipeline;

pub mod camera;
pub mod draw;
pub mod texture;
pub mod window;

pub struct Graphics {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub camera: camera::Camera,
    pub texture_pipeline: Arc<DefaultTexturePipeline>,
    pub clear_color: wgpu::Color,
}

pub struct SurfaceView {
    pub output: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
    pub encoder: wgpu::CommandEncoder,
    pub clear_color: wgpu::Color,
}

#[derive(Debug)]
pub enum GraphicsError {
    NoAdapter,
    RequestDeviceError,
}

impl Graphics {
    pub async fn new(window: &Window) -> Result<Self, GraphicsError> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or(GraphicsError::NoAdapter)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                None, // Trace path
            )
            .await
            .map_err(|_| GraphicsError::RequestDeviceError)?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        let camera = camera::Camera::new(&device);

        let texture_pipeline = DefaultTexturePipeline::new(&device, &config, &camera);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
            camera,
            texture_pipeline: Arc::new(texture_pipeline),
            clear_color: wgpu::Color {
                r: 0.1,
                g: 0.5,
                b: 0.2,
                a: 1.0,
            },
        })
    }

    pub(crate) fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn create_view(&self) -> Result<SurfaceView, wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        Ok(SurfaceView {
            view: output.texture.create_view(&Default::default()),
            output,
            encoder: self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Surface View"),
                }),
            clear_color: self.clear_color,
        })
    }
}

impl SurfaceView {
    pub fn draw(
        &mut self,
        label: Option<&str>,
        clear: Option<wgpu::Color>,
    ) -> wgpu::RenderPass {
        self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label,
            color_attachments: &[clear.map(|color| wgpu::RenderPassColorAttachment {
                view: &self.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        })
    }

    pub fn finish(self, queue: &wgpu::Queue) {
        queue.submit(std::iter::once(self.encoder.finish()));
        self.output.present();
    }
}
