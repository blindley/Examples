
fn main() {
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("fuzzy pickles");

    let wc = glutin::ContextBuilder::new().build_windowed(wb, &el).unwrap();
    let wc = unsafe { wc.make_current().unwrap() };

    gl::load_with(|p| wc.get_proc_address(p) as *const _);

    el.run(move |event, _, control_flow| {
        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;

        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => wc.resize(physical_size),
                _ => (),
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(0.6, 0.3, 0.4, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }

                wc.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}
