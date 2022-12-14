
use winit::{
  event::{
    Event,
    WindowEvent,
  },
  event_loop::{
    EventLoop,
    ControlFlow,
  },
  window::Window,
};

pub async fn run() {
  let event_loop = EventLoop::new();
  let window = Window::new(&event_loop).unwrap();
  let size = window.inner_size();

  let instance = wgpu::Instance::new(wgpu::Backends::METAL);
  let surface = unsafe { instance.create_surface(&window) };
  let adapter = instance
    .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(&surface)
    })
    .await
    .expect("Failed to find adapter");
  let (device, queue) = adapter
    .request_device(
      &wgpu::DeviceDescriptor {
        label: None,
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::default()
          .using_resolution(adapter.limits())
      },
      None
    )
    .await
    .expect("Failed to create device");
  let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: None,
    source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl"))),
  });
  let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: None,
    bind_group_layouts: &[],
    push_constant_ranges: &[],
  });
  let swapchain_format = surface.get_supported_formats(&adapter)[0];
  let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    label: None,
    layout: Some(&pipeline_layout),
    vertex: wgpu::VertexState {
      module: &shader,
      entry_point: "vs_main",
      buffers: &[],
    },
    fragment: Some(wgpu::FragmentState {
      module: &shader,
      entry_point: "fs_main",
      targets: &[Some(swapchain_format.into())],
    }),
    primitive: wgpu::PrimitiveState::default(),
    depth_stencil: None,
    multisample: wgpu::MultisampleState::default(),
    multiview: None,
  });
  let mut config = wgpu::SurfaceConfiguration {
    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    format: swapchain_format,
    width: size.width,
    height: size.height,
    present_mode: wgpu::PresentMode::Fifo,
    alpha_mode: surface.get_supported_alpha_modes(&adapter)[0],
  };

  surface.configure(&device, &config);

  event_loop.run(
    move | event, _, control_flow| {
      let _ = ();
      *control_flow = ControlFlow::Wait;
      match event {
        Event::WindowEvent {
          event: WindowEvent::CloseRequested,
          ..
        } => *control_flow = ControlFlow::Exit,
        Event::RedrawRequested(_) => {
          let frame = surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
          let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
          let mut encoder = device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
          {
            let mut render_pass = encoder
              .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                  view: &view,
                  resolve_target: None,
                  ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: true,
                  },
                })],
                depth_stencil_attachment: None,
              });
            render_pass.set_pipeline(&render_pipeline);
            render_pass.draw(0..3,0..1);
          }
          queue.submit(Some(encoder.finish()));
          frame.present();
        },
        Event::WindowEvent {
          event: WindowEvent::Resized(size),
          ..
        } => {
            config.width = size.width;
            config.height = size.height;
            surface.configure(&device, &config);
            window.request_redraw();
          }
        _ => {}
      }
    });

}


