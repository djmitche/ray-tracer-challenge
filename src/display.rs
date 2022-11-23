use crate::{spaces, Camera, Point, RayColor, Vector};
use softbuffer::GraphicsContext;
use winit::event::{DeviceEvent, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

/// Display the given world in a GUI window.
pub fn display(
    world: impl RayColor + 'static,
    fov: f64,
    from: Point<spaces::World>,
    to: Point<spaces::World>,
    up: Vector<spaces::World>,
    oversample: u32,
) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Ray Tracer Challenge")
        //.with_fullscreen(Some(Fullscreen::Borderless(None)))
        .build(&event_loop)
        .unwrap();
    let mut graphics_context = unsafe { GraphicsContext::new(window) }.unwrap();

    let mut display = false;

    let mut cur_width = 0;
    let mut cur_height = 0;
    let mut buffer = vec![0u32; 0];

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        let mut render = |graphics_context: &mut GraphicsContext<Window>| {
            let (width, height) = {
                let size = graphics_context.window().inner_size();
                (size.width, size.height)
            };

            // initially, just display black
            if !display {
                buffer = vec![0u32; (width * height) as usize];
                graphics_context.set_buffer(&buffer, width as u16, height as u16);
                return;
            }

            // render
            if width != cur_width || height != cur_height {
                buffer = vec![0x102030u32; (width * height) as usize];
                graphics_context.set_buffer(&buffer, width as u16, height as u16);
                let camera = Camera::new(width, height, fov, from, to, up, oversample);
                camera.u32_buffer(&world, &mut buffer[..]);
                cur_width = width;
                cur_height = height;
            }

            // display the result
            graphics_context.set_buffer(&buffer, width as u16, height as u16);
        };

        match event {
            Event::RedrawRequested(window_id) if window_id == graphics_context.window().id() => {
                render(&mut graphics_context);
            }
            Event::DeviceEvent {
                event: DeviceEvent::Key(key),
                ..
            } => match key.virtual_keycode {
                Some(VirtualKeyCode::Q) => *control_flow = ControlFlow::Exit,
                Some(VirtualKeyCode::Space) => {
                    display = true;
                    graphics_context.window().request_redraw();
                }
                _ => {}
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == graphics_context.window().id() => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                window_id,
            } if window_id == graphics_context.window().id() => {
                let x = position.x as u32;
                let y = position.y as u32;
                let (width, height) = {
                    let size = graphics_context.window().inner_size();
                    (size.width, size.height)
                };
                println!("getting color at ({}, {})", x, y);
                let camera = Camera::new(width, height, fov, from, to, up, oversample);
                println!("result: {:?}", camera.color_at(x, y, &world, true));
            }
            _ => {}
        }
    });
}
