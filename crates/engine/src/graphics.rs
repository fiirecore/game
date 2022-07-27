pub extern crate wgpu;
pub extern crate winit;

use winit::{window::Window, dpi::PhysicalSize};

pub mod texture;
pub mod window;

pub struct Graphics {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
}

pub struct SurfaceView {
    pub output: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
    pub encoder: wgpu::CommandEncoder,
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

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }).await.ok_or(GraphicsError::NoAdapter)?;

        
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
            .await.map_err(|_| GraphicsError::RequestDeviceError)?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
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
            encoder: self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Surface View"),
            }),
        })
        
    }

}

impl SurfaceView {

    pub fn finish(self, queue: &wgpu::Queue) {
        queue.submit(std::iter::once(self.encoder.finish()));
        self.output.present();
    }

}