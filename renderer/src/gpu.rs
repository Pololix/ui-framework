use crate::types::Viewport;
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum GpuStateError {
    #[error("Failed to create surface from target: {0}")]
    SurfaceCreation(#[from] wgpu::CreateSurfaceError),

    #[error("Failed to retrieve adapter: {0}")]
    AdapterRequest(#[from] wgpu::RequestAdapterError),

    #[error("Failed to retrieve device/queue pair: {0}")]
    DeviceRequest(#[from] wgpu::RequestDeviceError),
}

pub struct GpuState {
    _window: Arc<dyn wgpu::DisplayAndWindowHandle>,

    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,

    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl GpuState {
    pub fn new<W: wgpu::DisplayAndWindowHandle + 'static>(
        target: Arc<W>,
        viewport: Viewport,
    ) -> Result<Self, GpuStateError> {
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(target.clone())?;

        // obtain the gpu resources
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))?;
        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))?;

        // configure the surface
        let capabilities = surface.get_capabilities(&adapter);
        let format = capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            color_space: wgpu::SurfaceColorSpace::Auto,
            width: viewport.width,
            height: viewport.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Ok(Self {
            _window: target,

            surface,
            config,

            adapter,
            device,
            queue,
        })
    }

    pub fn set_viewport(&mut self, viewport: Viewport) {
        if viewport.width == 0 || viewport.height == 0 {
            return;
        }

        self.config.width = viewport.width;
        self.config.height = viewport.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn get_viewport(&mut self) -> Viewport {
        Viewport {
            width: self.config.width,
            height: self.config.height,
        }
    }
}
