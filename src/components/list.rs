use bounce::{use_atom_setter, use_selector_value};
use stylist::{self, style};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_size;
use yew_icons::{Icon, IconId};

use crate::{
  components::Image,
  store::{self, FilterImages, Size},
  utils::{download_action, style},
};

#[function_component]
pub fn List() -> Html {
  let class_name = get_class_name();
  let images = use_selector_value::<FilterImages>();
  let size = use_atom_setter::<Size>();
  let node = use_node_ref();
  let (width, _h) = use_size(node.clone());

  let animation_delay = |i: usize| -> String { format!("animation-delay: {}s", i as f32 * 0.2) };

  let combine_style = |style: Option<String>, i: usize| -> String {
    let style = style.map_or("".to_string(), |x| x);
    format!("{style}; transition-delay: {}s", i as f32 * 0.1)
  };

  let download = Callback::from(move |item: store::Image| {
    spawn_local(async {
      let _ = download_action(item).await;
    });
  });

  use_effect_with(width, move |val: &u32| {
    size(Size::new(*val as f64));
  });

  html! {
    <section class={class_name}>
      <div class="bk-list__wrap scroll-bar" ref={node}>
      {for images.value().iter().enumerate().map(|(i, item)| {
        let ele = item.clone();
        let ele_clone = item.clone();
        let width = ele.style_w.map_or(0.0, |x| x);
        let height = ele.style_h.map_or(0.0, |x| x);
        html!{
          <figure
            style={combine_style(ele.style, i)}
            key={ele.id}
            class={"bk-list__item"}
          >
          <Image
            alternative={Some("/image/error.png")}
            class_name="bk-list__img"
            width={width}
            height={height}
            style={animation_delay(i)}
            src={ele.preview}
          />
          <div class="bk-list__tool">
            <p class="bk-list__info">
              {ele.width} {" / "} {ele.height}
            </p>
          </div>
          <span
            class="bk-list__down"
            onclick={download.reform(move |_| ele_clone.clone())}
          >
            <Icon icon_id={IconId::FontAwesomeSolidDownload}  width="1em" height="1em" />
          </span>
          </figure>
        }})}
      </div>
    </section>
  }
}

#[allow(non_upper_case_globals)]
fn get_class_name() -> String {
  style::get_class_name(style!(
    r#"
    width: 100%;
    height: 100%;
    .bk-list__wrap {
      position: relative;
      background: none;
      width: 100%;
      height: 100%;
      overflow-y: auto;
    }
    
    .bk-list__item {
      margin: 0;
      border: calc(var(--theme-list-gap) / 2) solid transparent;
      box-sizing: border-box;
      overflow: hidden;
      position: absolute;
      transition: transform 0.5s ease-in-out;
      transform-origin: center;
      left: 0;
      top: 0;
      background-clip: content-box;
    }
    
    .bk-list__item:hover .bk-list__img,.bk-list__item:hover .bk-list__tool {
      transform: translateY(calc(var(--theme-list-tool-height) * -1));
    }

    .bk-list__item:hover .bk-list__down {
      transform: scale(1);
      opacity: 1;
      visibility: visible;
    }
    
    .bk-list__item .sk-spinner {
      color: var(--theme-base-color);
      position: absolute;
      left: 0;
      right: 0;
      top: 0;
      bottom: 0;
      margin: auto;
    }
    
    .bk-list__tool {
      position: absolute;
      display: flex;
      height: var(--theme-list-tool-height);
      width: 100%;
      justify-content: flex-start;
      flex-flow: row nowrap;
      align-items: stretch;
      transition: transform 0.2s ease;
      /* this is a performance issue  */
      -webkit-backdrop-filter: blur(10px);
              backdrop-filter: blur(10px);
      border-bottom: 1px solid var(--theme-base-color);
      box-sizing: border-box;
    }
    
    .bk-list__info {
      flex: 1 1 auto;
      font-size: 12px;
      color: var(--theme-base-color);
      line-height: var(--theme-list-tool-height);
      text-align: center;
      margin: 0;
      font-family: ZagRegular;
    }
    
    .bk-list__down {
      position: absolute;
      justify-content: center;
      align-items: center;
      width: var(--theme-list-tool-height);
      color: var(--theme-base-color);
      font-size: 40px;
      height: 40px;
      width: 40px;
      top: 0;
      bottom: 0;
      left: 0;
      right: 0;
      margin: auto;
      transform: scale(3);
      opacity: 0;
      visibility: hidden;
      overflow: hidden;
      transition: all 0.2s ease;
      cursor: pointer;
    }
    
    .bk-list__down i {
      flex: 0 0 auto;
    }
    
    .bk-list__img {
      display: block;
      width: 100%;
      box-sizing: border-box;
      height: 100%;
      border-radius: 2px;
      object-fit: cover;
      cursor: pointer;
      transition: transform 0.2s ease;
    }
    
    .bk-list__img.error {
      object-fit: contain;
    }
    
    .flip-enter {
      opacity: 0;
      transform: translateY(20px);
    }
    .flip-exit {
      opacity: 1;
      transform: translateY(0);
    }
    
    .flip-enter-active {
      transition: all 0.5s ease-in-out;
      transform: translateY(0);
      opacity: 1;
    }
    .flip-exit-active {
      opacity: 0;
      transform: translateY(20px);
      transition: all 0.5s ease-in-out;
    }
    "#
  ))
}
