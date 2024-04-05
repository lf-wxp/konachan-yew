use color_thief::Color;
use indexmap::{self, IndexMap};
use js_sys::ArrayBuffer;
use rand::{self, Rng};
use std::ops::Range;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
  window, Blob, BlobPropertyBag, CanvasRenderingContext2d, Document, Event, FileReader,
  HtmlAnchorElement, HtmlCanvasElement, HtmlImageElement, RegistrationOptions, Url, Window,
};
use yew::{
  virtual_dom::{ApplyAttributeAs, Attributes, VNode},
  AttrValue, NodeRef,
};

pub fn random(rang: Range<u16>) -> u16 {
  rand::thread_rng().gen_range(rang)
}

pub fn num_in_range(start: f64, end: f64, num: f64) -> f64 {
  if num <= start {
    return start;
  }
  if num >= end {
    return end;
  }
  num
}

pub fn get_window() -> Window {
  window().expect("no global `window` exists")
}

pub fn get_document() -> Document {
  get_window()
    .document()
    .expect("no global `Document` exists")
}

pub fn query_selector<T: JsCast>(selector: &str) -> Option<T> {
  get_document()
    .query_selector(selector)
    .ok()
    .and_then(|x| x)
    .and_then(|x| x.dyn_into::<T>().ok())
}

pub fn get_dpr() -> f64 {
  if let Some(w) = window() {
    return w.device_pixel_ratio();
  }
  1.0
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
  get_window()
    .request_animation_frame(f.as_ref().unchecked_ref())
    .expect("should register `requestAnimationFrame` OK");
}

pub fn class_name_determine(condition: bool, name: &str, append: &str) -> String {
  format!("{} {}", name, if condition { append } else { "" })
}

pub fn get_vnode_attr(vnode: VNode, attr: &str) -> String {
  match vnode {
    VNode::VTag(vtag) => vtag
      .attributes
      .iter()
      .find(|&(key, _value)| key == attr)
      .map_or("".to_string(), |(_key, val)| val.to_string()),
    _ => "".to_string(),
  }
}

pub fn append_vnode_attr(vnode: VNode, key: &'static str, val: String) -> VNode {
  let pre_val = get_vnode_attr(vnode.clone(), key);

  match vnode {
    VNode::VTag(mut vtag) => {
      let mut indexmap = IndexMap::new();
      indexmap.insert(
        AttrValue::from(key),
        (
          AttrValue::from(format!("{} {}", pre_val, val)),
          ApplyAttributeAs::Attribute,
        ),
      );
      let attr = Attributes::IndexMap(indexmap);
      vtag.set_attributes(attr);
      VNode::VTag(vtag)
    }
    _ => vnode.clone(),
  }
}

pub fn add_child(vnode: VNode, child: VNode) -> VNode {
  match vnode {
    VNode::VTag(mut vtag) => {
      vtag.add_child(child);
      VNode::VTag(vtag)
    }
    _ => vnode.clone(),
  }
}
pub fn get_target<T, H>(e: T) -> Option<H>
where
  T: AsRef<web_sys::Event>,
  H: JsCast,
{
  e.as_ref().target().and_then(|t| t.dyn_into::<H>().ok())
}

pub async fn read_file(file: web_sys::File) -> Result<js_sys::ArrayBuffer, JsValue> {
  let promise = js_sys::Promise::new(&mut |resolve, reject| {
    let file_reader = FileReader::new().unwrap();
    let file_reader_ok = file_reader.clone();
    let load = Closure::wrap(Box::new(move |_event: Event| {
      let array_buffer: js_sys::ArrayBuffer = file_reader_ok.result().unwrap().dyn_into().unwrap();
      let _ = resolve.call1(&JsValue::undefined(), &array_buffer);
    }) as Box<dyn FnMut(_)>);
    let error = Closure::wrap(Box::new(move |err: JsValue| {
      let _ = reject.call1(&JsValue::undefined(), &err);
    }) as Box<dyn FnMut(_)>);
    let _ = file_reader.add_event_listener_with_callback("load", load.as_ref().unchecked_ref());
    let _ = file_reader.add_event_listener_with_callback("error", error.as_ref().unchecked_ref());
    let _ = file_reader.read_as_array_buffer(&file);
    load.forget();
    error.forget();
  });
  let array_buffer = JsFuture::from(promise).await?;
  let array_buffer: js_sys::ArrayBuffer = array_buffer.dyn_into()?;
  Ok(array_buffer)
}

pub fn array_buffer_to_blob_url(
  array_buffer: &ArrayBuffer,
  mime_type: &str,
) -> Result<String, JsValue> {
  let array: js_sys::Array = js_sys::Array::new();
  array.push(array_buffer);
  let blob =
    Blob::new_with_u8_array_sequence_and_options(&array, BlobPropertyBag::new().type_(mime_type))?;

  let url = Url::create_object_url_with_blob(&blob)?;
  Ok(url)
}

pub fn get_ctx(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d, JsValue> {
  let ctx = canvas
    .get_context("2d")?
    .ok_or("")?
    .dyn_into::<CanvasRenderingContext2d>()
    .ok()
    .ok_or("")?;
  Ok(ctx)
}

pub fn get_html_image_to_vec(img: HtmlImageElement) -> Result<Vec<u8>, JsValue> {
  let document = get_document();
  let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
  canvas.set_width(img.width());
  canvas.set_height(img.height());
  let width = canvas.width() as f64;
  let height = canvas.height() as f64;
  let ctx = get_ctx(&canvas)?;
  ctx.draw_image_with_html_image_element(&img, 0.0, 0.0)?;
  let img_data = ctx.get_image_data(0.0, 0.0, width, height)?;
  Ok(img_data.data().to_vec())
}

pub fn node_ref_to_html<T: JsCast>(node_ref: NodeRef) -> Option<T> {
  node_ref.get().and_then(|node| node.dyn_into::<T>().ok())
}

pub fn bare_rgb(rgb: Color) -> String {
  let Color { r, g, b } = rgb;
  format!("{r},{g},{b}")
}

pub fn download_file(url: &str, name: &str) -> Result<(), JsValue> {
  let document = get_document();
  let body = document.body().ok_or("get body error")?;
  let a: HtmlAnchorElement = document.create_element("a").unwrap().dyn_into().unwrap();
  a.set_href(url);
  a.set_download(name);
  a.set_target("_blank");
  body.append_child(&a)?;
  a.click();
  body.remove_child(&a)?;
  Ok(())
}

pub fn register_ws() {
  let window = get_window();
  let closure = Closure::<dyn Fn(_)>::new(move |_: Event| {
    spawn_local(async move {
      let window = get_window();
      let mut options = RegistrationOptions::new();
      options.scope("/");
      let _ = JsFuture::from(
        window
          .navigator()
          .service_worker()
          .register_with_options("/sw.js", &options),
      )
      .await;
    });
  });
  window
    .add_event_listener_with_callback("load", closure.as_ref().unchecked_ref())
    .ok();
  closure.forget();
}
