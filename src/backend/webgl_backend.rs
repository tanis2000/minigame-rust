use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use stdweb::web::event as webevent;
use stdweb::web::event::{ConcreteEvent, IEvent, IKeyboardEvent, IMouseEvent, IUiEvent};
use stdweb::web::{self, html_element::CanvasElement, IEventTarget, IHtmlElement, IParentNode, CanvasRenderingContext2d, Document};
use stdweb::{unstable::TryInto, Reference};
use backend::backend::AbstractBackend;

#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Event")] // TODO: Better type check.
pub struct WheelEvent(Reference);

impl IEvent for WheelEvent {}
impl IUiEvent for WheelEvent {}
impl IMouseEvent for WheelEvent {}
impl ConcreteEvent for WheelEvent {
    const EVENT_TYPE: &'static str = "wheel";
}

struct WebGLBackendData {
  canvas: CanvasElement,
}

pub struct WebGLBackend {
  data: Rc<RefCell<WebGLBackendData>>,
  hidpi_factor: f64,
}

impl AbstractBackend for WebGLBackend {
  fn build() -> Self {
    stdweb::initialize();
    console!(log, "1");
    let hidpi_factor = 1.0;//js!{ return window.devicePixelRatio; }.try_into().unwrap();
    console!(log, "2");
    let d = web::document();
    console!(log, "ok 1");
    let c = d.query_selector("#canvas");
    match c {
      Ok(c) => {
        console!(log, "ok 2");
        console!(log, c);
      },
      Err(e) => {
        console!(log, "{}", &e.to_string());
      }
    } 

    let canvas: CanvasElement = web::document()
        .query_selector("#canvas")
        .expect("No canvas found.")
        .unwrap()
        .try_into()
        .unwrap();
    console!(log, "3");

    // NOTE: This might be no longer needed. 
    let web_ctxt: CanvasRenderingContext2d = canvas.get_context().unwrap();

    canvas.set_width((canvas.offset_width() as f64 * hidpi_factor) as u32);
    canvas.set_height((canvas.offset_height() as f64 * hidpi_factor) as u32);
    let data = Rc::new(RefCell::new(WebGLBackendData {
        canvas,
        //cursor_pos: None,
        //key_states: [Action::Release; Key::Unknown as usize + 1],
        //button_states: [Action::Release; MouseButton::Button8 as usize + 1],
        //pending_events: Vec::new(),
        //out_events,
    }));

    let edata = data.clone();
    let _ = web::window().add_event_listener(move |_: webevent::ResizeEvent| {
        let mut edata = edata.borrow_mut();
        let (w, h) = (
            (edata.canvas.offset_width() as f64 * hidpi_factor) as u32,
            (edata.canvas.offset_height() as f64 * hidpi_factor) as u32,
        );
        edata.canvas.set_width(w);
        edata.canvas.set_height(h);
        //let _ = edata
        //    .pending_events
        //    .push(WindowEvent::FramebufferSize(w, h));
        //let _ = edata.pending_events.push(WindowEvent::Size(w, h));
    });

    let edata = data.clone();
    let _ = web::window().add_event_listener(move |e: webevent::MouseDownEvent| {
        //let mut edata = edata.borrow_mut();
        //let button = translate_mouse_button(&e);
        //let _ = edata.pending_events.push(WindowEvent::MouseButton(
        //    button,
        //    Action::Press,
        //    translate_mouse_modifiers(&e),
        //));
        //edata.button_states[button as usize] = Action::Press;
    });

    let edata = data.clone();
    let _ = web::window().add_event_listener(move |e: webevent::MouseUpEvent| {
        let mut edata = edata.borrow_mut();
        //let button = translate_mouse_button(&e);
        //let _ = edata.pending_events.push(WindowEvent::MouseButton(
        //    button,
        //    Action::Release,
        //    translate_mouse_modifiers(&e),
        //));
        //edata.button_states[button as usize] = Action::Release;
    });

    let edata = data.clone();
    let _ = web::window().add_event_listener(move |e: webevent::MouseMoveEvent| {
        let mut edata = edata.borrow_mut();
        //edata.cursor_pos = Some((e.offset_x() as f64, e.offset_y() as f64));
        //let _ = edata.pending_events.push(WindowEvent::CursorPos(
        //    e.offset_x() as f64,
        //    e.offset_y() as f64,
        //    translate_mouse_modifiers(&e),
        //));
    });

    let edata = data.clone();
    let _ = web::window().add_event_listener(move |e: WheelEvent| {
        let delta_x: i32 = js!(
            return @{e.as_ref()}.deltaX;
        ).try_into()
            .ok()
            .unwrap_or(0);
        let delta_y: i32 = js!(
            return @{e.as_ref()}.deltaY;
        ).try_into()
            .ok()
            .unwrap_or(0);
        let mut edata = edata.borrow_mut();
        //let _ = edata.pending_events.push(WindowEvent::Scroll(
        //    delta_x as f64,
        //    delta_y as f64,
        //    translate_mouse_modifiers(&e),
        //));
    });

    let edata = data.clone();
    let _ = web::window().add_event_listener(move |e: webevent::KeyDownEvent| {
        let mut edata = edata.borrow_mut();
        //let key = translate_key(&e);
        //let _ = edata.pending_events.push(WindowEvent::Key(
        //    key,
        //    Action::Press,
        //    translate_key_modifiers(&e),
        //));
        //edata.key_states[key as usize] = Action::Press;
    });

    let edata = data.clone();
    let _ = web::window().add_event_listener(move |e: webevent::KeyUpEvent| {
        let mut edata = edata.borrow_mut();
        //let key = translate_key(&e);
        //let _ = edata.pending_events.push(WindowEvent::Key(
        //    key,
        //    Action::Release,
        //    translate_key_modifiers(&e),
        //));
        //edata.key_states[key as usize] = Action::Release;
    });

    WebGLBackend {
      data,
      hidpi_factor,
    }
  }

  fn swap_buffers(&mut self) {
    // Nothing to do
  }

  fn poll_events(&mut self) {
    let mut data_borrow = self.data.borrow_mut();
    let data = data_borrow.deref_mut();

    //for e in data.pending_events.drain(..) {
    //    let _ = data.out_events.send(e);
    //}
  }

  fn size(&self) -> (u32, u32) {
    (
      self.data.borrow().canvas.offset_width() as u32,
      self.data.borrow().canvas.offset_height() as u32,
    )
  }

}