use crate::{
    RenderCommand, RenderId,
    frame::Frame,
    gpu::{GpuState, GpuStateError},
    quad::Quad,
    types::{Color, Viewport},
};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, thiserror::Error)]
pub enum RendererError {
    #[error("{0}")]
    GpuState(#[from] GpuStateError),
}

pub struct Renderer {
    gpu: GpuState,
    frame: Frame,
    cmd_queue: Vec<RenderCommand>,

    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,

    viewport_buffer: wgpu::Buffer,
    scale_factor_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    instance_buffer_capacity: usize,
}

impl Renderer {
    pub fn new<W: wgpu::DisplayAndWindowHandle + 'static>(
        target: Arc<W>,
        viewport: Viewport,
    ) -> Result<Self, RendererError> {
        let gpu = GpuState::new(target, viewport)?;

        let instance_buffer_capacity = 256;
        let instance_buffer = Quad::instance_buffer(&gpu.device, instance_buffer_capacity);

        let viewport_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Viewport buffer"),
            size: std::mem::size_of::<[f32; 2]>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let scale_factor_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Scale factor buffer"),
            size: std::mem::size_of::<f32>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout =
            gpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Main bind group layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::VERTEX,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::VERTEX,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });

        let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Main bind group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: viewport_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: scale_factor_buffer.as_entire_binding(),
                },
            ],
        });

        let shader = gpu
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Main shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Main render pipeline layout"),
                bind_group_layouts: &[Some(&bind_group_layout)],
                immediate_size: 0,
            });

        let pipeline = gpu
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Main render pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &[Some(Quad::instance_buffer_layout())],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: gpu.config.format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview_mask: None,
                cache: None,
            });

        Ok(Self {
            gpu,
            frame: Frame::default(),
            cmd_queue: Vec::new(),

            pipeline,
            bind_group,

            viewport_buffer,
            scale_factor_buffer,
            instance_buffer,
            instance_buffer_capacity,
        })
    }

    pub fn submit(&mut self, cmds: &[RenderCommand]) {
        self.cmd_queue.extend_from_slice(cmds);
    }

    pub fn dispatch(&mut self) {
        if self.cmd_queue.is_empty() {
            return;
        }

        let mut quads_by_id: HashMap<RenderId, Vec<Quad>> = HashMap::new();
        let cmds: Vec<_> = self.cmd_queue.drain(..).collect();

        // everything converges into quads for simple instanced rendering
        for cmd in cmds {
            match cmd {
                // globals
                RenderCommand::Resize(viewport) => {
                    self.gpu.set_viewport(viewport);
                    self.update_viewport_buffer(viewport);
                    self.frame.full_invalidation();
                }
                RenderCommand::ChangeScaleFactor(scale_factor) => {
                    self.update_scale_factor_buffer(scale_factor);
                    self.frame.full_invalidation();
                }
                RenderCommand::RedrawFrame => {
                    self.frame.full_invalidation();
                }
                RenderCommand::ClearFrame => {
                    self.frame.clear();
                    self.frame.full_invalidation();
                }

                // semantic elements
                RenderCommand::Quad { .. } => {}

                // removal
                RenderCommand::Remove(id) => self.frame.remove(id),
            }
        }

        // upload quads to the frame
        quads_by_id.iter().for_each(|(id, quads)| {
            self.frame.upload(*id, quads);
        });

        self.draw();
    }

    fn draw(&mut self) {
        let Some((invalid_rect, quads)) = self.frame.get_quads(self.gpu.get_viewport()) else {
            return; // no redraw needed if empty invalidation
        };

        let texture = match self.gpu.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture)
            | wgpu::CurrentSurfaceTexture::Suboptimal(texture) => texture,

            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.gpu.reconfigure();
                return;
            }

            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                return;
            }
        };

        let view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Main encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Main render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);

        // clear the invalid area first
        let clear_quad = Quad::from_rect(invalid_rect, Color::BLACK);
        self.gpu
            .queue
            .write_buffer(&self.instance_buffer, 0, bytemuck::bytes_of(&clear_quad));

        render_pass.set_vertex_buffer(0, self.instance_buffer.slice(..));
        render_pass.draw(0..4, 0..1);

        // if necessary, update buffer capacity by creating a new buffer with a bigger capacity
        if !quads.is_empty() {
            if quads.len() > self.instance_buffer_capacity {
                self.instance_buffer_capacity = quads.len().next_power_of_two();
                self.instance_buffer =
                    Quad::instance_buffer(&self.gpu.device, self.instance_buffer_capacity);
            }

            self.gpu
                .queue
                .write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&quads));

            render_pass.set_vertex_buffer(0, self.instance_buffer.slice(..));
            render_pass.draw(0..4, 0..quads.len() as u32);
        }

        drop(render_pass);
        self.gpu.queue.submit(Some(encoder.finish()));
        self.gpu.queue.present(texture);
    }

    fn update_viewport_buffer(&mut self, viewport: Viewport) {
        self.gpu.queue.write_buffer(
            &self.viewport_buffer,
            0,
            bytemuck::cast_slice(&[viewport.width as f32, viewport.height as f32]),
        );
    }

    fn update_scale_factor_buffer(&mut self, scale_factor: f32) {
        self.gpu.queue.write_buffer(
            &self.scale_factor_buffer,
            0,
            bytemuck::bytes_of(&scale_factor),
        );
    }
}
