//! A light abstraction over Wgpu rendering.

use std::path::{Path, PathBuf};

use wgpu::{
    Adapter,
    Backends,
    Device,
    DeviceDescriptor,
    Features,
    Instance,
    InstanceDescriptor,
    Limits,
    PowerPreference,
    Queue,
    RequestAdapterOptions,
    RequestDeviceError,
    TextureFormat,
};

/// A low-level abstsraction over Wgpu.
pub struct Backend {
    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    format: TextureFormat,
}

/// A builder interface for configuring setup of a [`Renderer`].
pub struct RendererBuilder<'a> {
    format: TextureFormat,

    instance: InstanceDescriptor,
    adapter: RequestAdapterOptions<'a>,

    device: DeviceDescriptor<'a>,
    trace_path: Option<PathBuf>,
}

/// Errors that can arise from creating a [`Renderer`].
#[derive(Debug, thiserror::Error)]
pub enum NewError {
    #[error("no suitable adapter found for configuration {0}")]
    NoAdapterFound(String),
    #[error(transparent)]
    RequestDeviceError(#[from] RequestDeviceError),
}

/// Errors that can occur when rendering.
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("idk")]
    Placeholder,
}

impl Backend {
    /// A new backend with default settings.
    ///
    /// See also [`Renderer::builder`].
    ///
    /// ## Errors
    ///
    /// See [`NewError`].
    pub async fn new() -> Result<Self, NewError> {
        Self::builder().build().await
    }

    /// A new backend using the provided texture format.
    ///
    /// ## Errors
    ///
    /// See [`NewError`].
    pub async fn new_using(format: TextureFormat) -> Result<Self, NewError> {
        Self::builder_using(format).build().await
    }

    /// Return a builder for configuring the backend.
    #[must_use]
    pub fn builder<'a>() -> RendererBuilder<'a> {
        Self::builder_using(TextureFormat::Rgba8UnormSrgb)
    }

    /// Returns a builder using the provided texture format.
    #[must_use]
    pub fn builder_using<'a>(format: TextureFormat) -> RendererBuilder<'a> {
        RendererBuilder::new(format)
    }

    /// The instance of Wgpu.
    #[must_use]
    pub fn instance(&self) -> &Instance {
        &self.instance
    }

    /// Adapter to the GPU.
    #[must_use]
    pub fn adapter(&self) -> &Adapter {
        &self.adapter
    }

    /// A handle to the GPU.
    #[must_use]
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// A queue of actions to be performed on the gpu.
    #[must_use]
    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    /// The texture format this renderer uses.
    #[must_use]
    pub fn format(&self) -> TextureFormat {
        self.format
    }
}

impl<'a> RendererBuilder<'a> {
    #[must_use]
    pub fn new(format: TextureFormat) -> Self {
        let instance = InstanceDescriptor::default();
        let adapter = RequestAdapterOptions::default();

        let device = DeviceDescriptor::default();
        let trace_path = None;

        Self { format, instance, adapter, device, trace_path }
    }

    // instance ---

    #[must_use]
    pub fn instance(self, instance: InstanceDescriptor) -> Self {
        Self { instance, ..self }
    }

    #[must_use]
    pub fn backends(self, backends: Backends) -> Self {
        let instance = InstanceDescriptor { backends, ..self.instance };

        Self { instance, ..self }
    }

    // adapter ---

    #[must_use]
    pub fn adapter(self, adapter: RequestAdapterOptions<'a>) -> Self {
        Self { adapter, ..self }
    }

    #[must_use]
    pub fn power_preference(self, power_preference: PowerPreference) -> Self {
        let adapter = RequestAdapterOptions { power_preference, ..self.adapter };

        Self { adapter, ..self }
    }

    #[must_use]
    pub fn force_fallback_adapter(self, force_fallback_adapter: bool) -> Self {
        let adapter = RequestAdapterOptions { force_fallback_adapter, ..self.adapter };

        Self { adapter, ..self }
    }

    // device ---

    /// Configure the device.
    #[must_use]
    pub fn device(self, device: DeviceDescriptor<'a>) -> Self {
        Self { device, ..self }
    }

    /// Set a label for the device for debugging purposes.
    #[must_use]
    pub fn device_label(self, label: &'a str) -> Self {
        let device = DeviceDescriptor { label: Some(label), ..self.device };

        Self { device, ..self }
    }

    /// Set the features of the device.
    ///
    /// See [`Features`].
    #[must_use]
    pub fn features(self, features: Features) -> Self {
        let device = DeviceDescriptor { features, ..self.device };

        Self { device, ..self }
    }

    /// Set device limits.
    ///
    /// See [`Limits`].
    #[must_use]
    pub fn limits(self, limits: Limits) -> Self {
        let device = DeviceDescriptor { limits, ..self.device };

        Self { device, ..self }
    }

    /// Trace path for logging.
    #[must_use]
    pub fn trace_path(self, trace_path: impl AsRef<Path>) -> Self {
        let trace_path = Some(trace_path.as_ref().to_owned());

        Self { trace_path, ..self }
    }

    /// Build the renderer.
    ///
    /// ## Errors
    ///
    /// See [`NewError`].
    pub async fn build(self) -> Result<Backend, NewError> {
        use NewError::NoAdapterFound;

        let Self { instance, adapter, device, trace_path, format } = self;

        let instance = Instance::new(instance);
        let adapter = instance
            .request_adapter(&adapter)
            .await
            .ok_or_else(|| NoAdapterFound(format!("{:?}", adapter)))?;

        let (device, queue) =
            adapter.request_device(&device, trace_path.as_deref()).await?;

        Ok(Backend { instance, adapter, device, queue, format })
    }
}
