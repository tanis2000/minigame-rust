use glutin::{ContextTrait, EventsLoop, WindowedContext};
use backend::backend::AbstractBackend;
use engine::gl as gl;
use std::ffi::{CStr};
use std::sync::mpsc::Sender;

pub struct GLBackend {
  window: WindowedContext,
  events: EventsLoop,
  //out_events: Sender<WindowEvent>,
}

impl AbstractBackend for GLBackend {
  fn build() -> Self {
    let mut el = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new()
    .with_title("minigame-rust")
    .with_dimensions(glutin::dpi::LogicalSize::new(800.0, 600.0));

    let windowed_context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_gl_profile(glutin::GlProfile::Compatibility)
        //.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGlEs, (3, 0)))
        .build_windowed(wb, &el)
        .unwrap();

    unsafe { windowed_context.make_current().unwrap() };

    println!("Loading GL extensions");
    gl::load_with(|s| windowed_context.context().get_proc_address(s) as *const _);

    let version = unsafe {
    let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
        .to_bytes()
        .to_vec();
        String::from_utf8(data).unwrap()
    };

    println!("OpenGL version {}", version);

    let glsl_version = unsafe {
    let data = CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const _)
        .to_bytes()
        .to_vec();
        String::from_utf8(data).unwrap()
    };

    println!("GLSL version {}", glsl_version);

    GLBackend {
      window: windowed_context,
      events: el,
    }
  }

  fn swap_buffers(&mut self) {
    let _ = self.window.swap_buffers();
  }

  fn poll_events(&mut self) {
    //let out_events = &mut self.out_events;
    let window = &mut self.window;
    //let button_states = &mut self.button_states;
    //let key_states = &mut self.key_states;
    //let cursor_pos = &mut self.cursor_pos;

      self.events.poll_events(|event| match event {
          glutin::Event::WindowEvent { event, .. } => match event {
              glutin::WindowEvent::CloseRequested => {
                  //let _ = out_events.send(WindowEvent::Close);
              }
              glutin::WindowEvent::Resized(logical_size) => {
                  let dpi_factor = window.get_hidpi_factor();
                  let physical_size = logical_size.to_physical(dpi_factor);
                  window.resize(physical_size);
                  let fb_size: (u32, u32) = physical_size.into();
                  //let _ = out_events.send(WindowEvent::FramebufferSize(fb_size.0, fb_size.1));
              }
              glutin::WindowEvent::CursorMoved {
                  position,
                  modifiers,
                  ..
              } => {
                  //let modifiers = translate_modifiers(modifiers);
                  let dpi_factor = window.get_hidpi_factor();
                  let physical_pos = position.to_physical(dpi_factor);
                  //*cursor_pos = Some(physical_pos.into());
                  //let _ = out_events.send(WindowEvent::CursorPos(
                  //    physical_pos.x,
                  //    physical_pos.y,
                  //    modifiers,
                  //));
              }
              glutin::WindowEvent::MouseInput {
                  state,
                  button,
                  modifiers,
                  ..
              } => {
                /*
                  let action = translate_action(state);
                  let button = translate_mouse_button(button);
                  let modifiers = translate_modifiers(modifiers);
                  button_states[button as usize] = action;
                  let _ = out_events.send(WindowEvent::MouseButton(button, action, modifiers));
                  */
              }
              glutin::WindowEvent::MouseWheel {
                  delta, modifiers, ..
              } => {
                  let (x, y) = match delta {
                      glutin::MouseScrollDelta::LineDelta(dx, dy) => (dx as f64, dy as f64),
                      glutin::MouseScrollDelta::PixelDelta(delta) => delta.into(),
                  };
                  //let modifiers = translate_modifiers(modifiers);
                  //let _ = out_events.send(WindowEvent::Scroll(x, y, modifiers));
              }
              glutin::WindowEvent::KeyboardInput { input, .. } => {
                /*
                  let action = translate_action(input.state);
                  let key = translate_key(input.virtual_keycode);
                  let modifiers = translate_modifiers(input.modifiers);
                  key_states[key as usize] = action;
                  let _ = out_events.send(WindowEvent::Key(key, action, modifiers));
                  */
              }
              _ => {}
          },
          _ => {}
      })
    }

    fn size(&self) -> (u32, u32) {
      let hidpi = self.window.get_hidpi_factor();
      let logical_size = self
        .window
        .get_inner_size()
        .expect("The window was closed.");
      logical_size.to_physical(hidpi).into()
    }
}