pub mod pipeline;
pub mod vertex;

use std::{num::NonZeroU32, sync::Arc};

use image::GenericImageView;

use self::pipeline::DefaultTexturePipeline;

use super::{draw::Draw, Graphics};

#[derive(Debug, Clone)]
pub struct Texture(Arc<TextureBase>, Arc<DefaultTexturePipeline>);

#[derive(Debug)]
pub struct TextureBase {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub diffuse_bind_group: wgpu::BindGroup,
    pub camera_bind_group: Arc<wgpu::BindGroup>,
}

impl Texture {
    pub fn from_bytes(graphics: &Graphics, bytes: &[u8]) -> Result<Self, image::ImageError> {
        Ok(Self(
            Arc::new(TextureBase::new(
                graphics,
                &image::load_from_memory(bytes)?,
                None,
            )),
            graphics.texture_pipeline.clone(),
        ))
    }
}

impl TextureBase {
    pub fn new(
        graphics: &Graphics,
        image: &image::DynamicImage,
        label: Option<&str>,
    ) -> Self {

        let device = &graphics.device;
        let queue = &graphics.queue;

        let rgba8 = image.to_rgba8();
        let (width, height) = image.dimensions();

        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba8,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * width),
                rows_per_image: NonZeroU32::new(height),
            },
            size,
        );

        let view = texture.create_view(&Default::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &graphics.texture_pipeline.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                }
            ],
        });

        Self {
            texture,
            view,
            sampler,
            diffuse_bind_group,
            camera_bind_group: graphics.camera.bind_group.clone(),
        }
    }
}

impl Drop for TextureBase {
    fn drop(&mut self) {
        self.texture.destroy();
    }
}

impl<'a> Draw<'a> {
    pub fn texture(&mut self, texture: &'a Texture, x: f32, y: f32) {
        self.0.set_pipeline(&texture.1.render_pipeline);
        self.0.set_bind_group(0, &texture.0.diffuse_bind_group, &[]);
        self.0.set_bind_group(1, &texture.0.camera_bind_group, &[]);
        self.0
            .set_vertex_buffer(0, texture.1.vertex_buffer.slice(..));
        self.0
            .set_index_buffer(texture.1.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        self.0.draw_indexed(0..texture.1.indices.len() as u32, 0, 0..1);
    }

    pub fn finish(self) {}
}
