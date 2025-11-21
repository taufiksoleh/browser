//! Main Renderer using wgpu

use crate::render::gpu::{create_rect_vertices, Uniforms, Vertex, SHADER_SOURCE};
use crate::render::{DisplayCommand, DisplayList};
use crate::ui::Window;
use wgpu::util::DeviceExt;

/// GPU Renderer
pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    size: (u32, u32),
}

impl Renderer {
    /// Create a new renderer for the given window
    pub fn new(window: &Window) -> Result<Self, String> {
        pollster::block_on(Self::new_async(window))
    }

    async fn new_async(window: &Window) -> Result<Self, String> {
        let size = window.inner_size();

        // Create wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Create surface
        let surface = instance
            .create_surface(window.raw_window())
            .map_err(|e| format!("Failed to create surface: {}", e))?;

        // Get adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or("Failed to get GPU adapter")?;

        // Create device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: Some("Browser GPU Device"),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .map_err(|e| format!("Failed to create device: {}", e))?;

        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // Create shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
        });

        // Create uniform buffer
        let uniforms = Uniforms::new(size.0 as f32, size.1 as f32);
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Uniform Bind Group Layout"),
        });

        // Create bind group
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("Uniform Bind Group"),
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Ok(Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            uniform_buffer,
            uniform_bind_group,
            size,
        })
    }

    /// Resize the renderer
    pub fn resize(&mut self, new_size: (u32, u32)) {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0;
            self.config.height = new_size.1;
            self.surface.configure(&self.device, &self.config);

            // Update uniforms
            let uniforms = Uniforms::new(new_size.0 as f32, new_size.1 as f32);
            self.queue
                .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
        }
    }

    /// Render a display list
    pub fn render(&mut self, display_list: &DisplayList) -> Result<(), String> {
        let output = self
            .surface
            .get_current_texture()
            .map_err(|e| format!("Failed to get surface texture: {}", e))?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Collect all vertices and indices
        let (vertices, indices) = self.build_geometry(display_list);

        if !vertices.is_empty() {
            let vertex_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

            let index_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(&indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 1.0,
                                g: 1.0,
                                b: 1.0,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
            }
        } else {
            // Just clear the screen
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// Build geometry from display list
    fn build_geometry(&self, display_list: &DisplayList) -> (Vec<Vertex>, Vec<u16>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for item in display_list.items() {
            match &item.command {
                DisplayCommand::SolidRect { rect, color } => {
                    let base_index = vertices.len() as u16;
                    let (rect_verts, rect_indices) =
                        create_rect_vertices(rect.x, rect.y, rect.width, rect.height, *color);
                    vertices.extend_from_slice(&rect_verts);
                    indices.extend(rect_indices.iter().map(|i| i + base_index));
                }
                DisplayCommand::Text {
                    text,
                    x,
                    y,
                    color,
                    font_size,
                } => {
                    // Simple text rendering as rectangles (placeholder)
                    // Real implementation would use font rasterization
                    let char_width = font_size * 0.6;
                    let mut current_x = *x;
                    for _ in text.chars() {
                        let base_index = vertices.len() as u16;
                        let (char_verts, char_indices) = create_rect_vertices(
                            current_x,
                            *y - font_size,
                            char_width,
                            *font_size,
                            *color,
                        );
                        vertices.extend_from_slice(&char_verts);
                        indices.extend(char_indices.iter().map(|i| i + base_index));
                        current_x += char_width;
                    }
                }
                DisplayCommand::Border {
                    rect,
                    widths,
                    colors,
                } => {
                    // Top border
                    if widths.0 > 0.0 {
                        let base_index = vertices.len() as u16;
                        let (verts, idx) =
                            create_rect_vertices(rect.x, rect.y, rect.width, widths.0, colors.0);
                        vertices.extend_from_slice(&verts);
                        indices.extend(idx.iter().map(|i| i + base_index));
                    }
                    // Right border
                    if widths.1 > 0.0 {
                        let base_index = vertices.len() as u16;
                        let (verts, idx) = create_rect_vertices(
                            rect.x + rect.width - widths.1,
                            rect.y,
                            widths.1,
                            rect.height,
                            colors.1,
                        );
                        vertices.extend_from_slice(&verts);
                        indices.extend(idx.iter().map(|i| i + base_index));
                    }
                    // Bottom border
                    if widths.2 > 0.0 {
                        let base_index = vertices.len() as u16;
                        let (verts, idx) = create_rect_vertices(
                            rect.x,
                            rect.y + rect.height - widths.2,
                            rect.width,
                            widths.2,
                            colors.2,
                        );
                        vertices.extend_from_slice(&verts);
                        indices.extend(idx.iter().map(|i| i + base_index));
                    }
                    // Left border
                    if widths.3 > 0.0 {
                        let base_index = vertices.len() as u16;
                        let (verts, idx) =
                            create_rect_vertices(rect.x, rect.y, widths.3, rect.height, colors.3);
                        vertices.extend_from_slice(&verts);
                        indices.extend(idx.iter().map(|i| i + base_index));
                    }
                }
                _ => {}
            }
        }

        (vertices, indices)
    }

    /// Get current size
    pub fn size(&self) -> (u32, u32) {
        self.size
    }
}
