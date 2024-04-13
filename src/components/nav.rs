use bounce::{use_atom, use_atom_value, use_slice};
use gloo_console::log;
use stylist::{self, style};
use tauri_sys::window;
use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;
use yew_hooks::use_effect_once;

use crate::{
  store::{Page, PageAction},
  utils::{get_window, style},
};

const SIZE: u32 = 4;

fn get_page_vec(page: u32, pages: u32) -> Vec<u32> {
  if pages == 0 {
    return Vec::new();
  }
  let half = SIZE / 2;
  let mut navpage = Vec::new();
  if page > half && page < pages - half {
    let mut i = page - half;
    let mut j = 0;
    while j < SIZE {
      navpage.push(i);
      i += 1;
      j += 1;
    }
  }
  if page <= half {
    let mut i = 1;
    let mut j = 0;
    while j < SIZE {
      navpage.push(i);
      j += 1;
      i += 1;
    }
  }
  if page >= pages - half {
    let mut i = pages - SIZE + 1;
    let mut j = 0;
    while j < SIZE {
      navpage.push(i);
      j += 1;
      i += 1;
    }
  }
  navpage
}

#[function_component]
pub fn Nav() -> Html {
  let class_name = get_class_name();
  let page = use_slice::<Page>();
  let page_vec = get_page_vec(page.current as u32, page.total as u32);

  let page_clone = page.clone();
  let next = Callback::from(move |_: MouseEvent| {
    page_clone.dispatch(PageAction::Next);
  });
  let page_clone = page.clone();
  let prev = Callback::from(move |_: MouseEvent| {
    page_clone.dispatch(PageAction::Prev);
  });
  let page_clone = page.clone();
  let invoke = Callback::from(move |id: u32| {
    page_clone.dispatch(PageAction::Invoke(id as usize));
  });
  let goto = Callback::from(move |_: MouseEvent| {});
  let change = Callback::from(move |_: Event| {});

  let bk_page = {
    let param = if !page_vec.is_empty() { "active" } else { "" };
    format!("bk-pager {}", param)
  };

  let bk_page_nav_prev = {
    let result = page.current.saturating_sub(1);
    let param = if result > 0 { "" } else { "disabled" };
    format!("bk-pager_nav {}", param)
  };

  let bk_page_nav_next = {
    let result = page.total.saturating_sub(page.current);
    let param = if result > 0 { "" } else { "disabled" };
    format!("bk-pager_nav {}", param)
  };

  let bk_page_item = |item: &u32| -> String {
    let current = if page.current == (*item) as usize {
      "current"
    } else {
      ""
    };
    let size = if page.current >= 102 { "middle" } else { "" };
    format!("bk-pager_item {} {}", current, size)
  };

  let page_clone = page.clone();
  use_effect_once(move || {
    let window = get_window();
    let closure = Closure::<dyn Fn(_)>::new(move |e: KeyboardEvent| {
      match e.key_code() {
        37 => {
          page_clone.dispatch(PageAction::Prev);
        }
        39 => {
          page_clone.dispatch(PageAction::Next);
        }
        _ => (),
      };
    });
    window
      .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
      .ok();
    closure.forget();
    || {}
  });

  html! {
    <section class={class_name}>
      <div class={bk_page}>
        <span
          class={bk_page_nav_prev}
          onclick={prev}
          role="button"
        >
          <i />
        </span>
        <span
          class={bk_page_nav_next}
          onclick={next}
          role="button"
        >
          <i />
        </span>
        <div class="bk-pager_con">
          <ul class="bk-pager_box">
            {for page_vec.iter().map(|item| {
              let i = *item;
              html!{
                <li
                  class={bk_page_item(item)}
                  onclick={invoke.clone().reform(move |_| i)}
                  key={*item}
                  data-id={(*item).to_string()}
                  role="button"
                >
                  <span class="bk-pager_item-text">{item}</span>
                </li>
              }
            })}
          </ul>
        </div>
        <form class="bk-pager_go">
          <em class="bk-pager_go-em" />
          <div class="bk-pager_go-div">
            <span class="bk-pager_go-span">{page.total}</span>
          </div>
          <div class="bk-pager_go-div">
            <input
              class={"bk-pager_go-input animation"}
              type="text"
              placeholder="page"
              name="pager"
              value={page.current.to_string()}
              onchange={change}
            />
          </div>
          <button class="bk-pager_btn" onclick={goto}>
            <span />
          </button>
        </form>
        <div class="bk-pager_holder" />
      </div>
    </section>
  }
}

#[allow(non_upper_case_globals)]
fn get_class_name() -> String {
  style::get_class_name(style!(
    r#"
    .bk-pager {
      flex: 0 0 auto;
      width: calc(3 * var(--theme-page-item-size));
      height: calc(3 * var(--theme-page-item-size));
      position: relative;
      z-index: 2;
      font-family: ZagRegular;
    }
    
    .bk-pager.active {
      animation: none;
    }
    
    .bk-pager.active .bk-pager_nav:first-of-type {
      transform: translate(-100%, 100%);
    }
    
    .bk-pager.active .bk-pager_nav:last-of-type {
      transform: translate(100%, -100%);
    }
    
    .bk-pager.active .bk-pager_go .bk-pager_go-div:first-of-type {
      left: var(--theme-page-item-size);
      top: 0px;
    }
    
    .bk-pager.active .bk-pager_go .bk-pager_go-div:last-of-type {
      top: var(--theme-page-item-size);
      left: 0px;
      transition-delay: 0.1s;
    }
    
    .bk-pager.active .bk-pager_go .bk-pager_btn {
      top: var(--theme-page-item-size);
      left: var(--theme-page-item-size);
      transition-delay: 0.2s;
    }
    
    .bk-pager.active .bk-pager_holder {
      visibility: hidden;
      opacity: 0;
    }
    
    .bk-pager.active .bk-pager_item:nth-child(1) {
      left: 0px;
      top: 0px;
    }

    .bk-pager.active .bk-pager_item:nth-child(2) {
      left: var(--theme-page-item-size);
      top: 0px;
      transition-delay: 0.1s;
    }
    
    .bk-pager.active .bk-pager_item:nth-child(3) {
      left: 0px;
      top: var(--theme-page-item-size);
      transition-delay: 0.2s;
    }
    
    .bk-pager_nav {
      width: var(--theme-page-item-size);
      height: var(--theme-page-item-size);
      position: absolute;
      display: inline-block;
      color: white;
      font-size: 30px;
      line-height: 40px;
      text-align: center;
      background-color: var(--theme-page-nav-bg-color);
      cursor: pointer;
      transition: all 0.3s ease;
      bottom: var(--theme-page-item-size);
      left: var(--theme-page-item-size);
    }
    
    .bk-pager_nav:hover:after,.bk-pager_nav:hover:before {
      background: var(--theme-page-item-hover-color) !important;
    }
    
    .bk-pager_nav svg {
      width: 100%;
      height: 100%;
      display: block;
    }
    
    .bk-pager_nav:after,.bk-pager_nav:before {
      transition: all 0.2s ease;
    }
    
    .bk-pager_nav:nth-of-type(1):after {
      content: '';
      position: absolute;
      left: 5px;
      bottom: 0;
      height: 5px;
      width: calc(100% - 5px);
      background: var(--theme-page-item-hover-color);
    }
    
    .bk-pager_nav:nth-of-type(1):before {
      content: '';
      position: absolute;
      left: 0;
      bottom: 0;
      height: 100%;
      width: 5px;
      background: var(--theme-page-item-hover-color);
    }
    
    .bk-pager_nav:nth-of-type(2):after {
      content: '';
      position: absolute;
      right: 5px;
      top: 0;
      height: 5px;
      width: calc(100% - 5px);
      background: var(--theme-page-item-hover-color);
    }

    .bk-pager_nav:nth-of-type(2):before {
      content: '';
      position: absolute;
      right: 0;
      top: 0;
      height: 100%;
      width: 5px;
      background: var(--theme-page-item-hover-color);
    }
    
    .bk-pager_nav.disabled {
      pointer-events: none;
      cursor: not-allowed;
    }
    
    .bk-pager_holder {
      width: var(--theme-page-item-size);
      height: var(--theme-page-item-size);
      position: absolute;
      background-color: var(--theme-base-color);
      z-index: 3;
      left: var(--theme-page-item-size);
      top: var(--theme-page-item-size);
      cursor: pointer;
      transition: all 0.2s 0.5s ease-in-out;
      animation: breathPage 2s 4s ease-in-out alternate infinite;
    }
    
    .bk-pager_holder:after,.bk-pager_holder:before {
      content: '';
      top: 0;
      right: 0;
      bottom: 0;
      left: 0;
      margin: auto;
      position: absolute;
    }
  
    .bk-pager_holder:after {
      width: 40%;
      height: 40%;
      background-color: var(--theme-page-item-breath-color2);
      animation: breathPage2 2s ease-in-out alternate infinite;
    }
    
    .bk-pager_holder:before {
      width: 70%;
      height: 70%;
      background-color: var(--theme-page-item-breath-color1);
      animation: breathPage1 2s 2s ease-in-out alternate infinite;
    }
    
    .bk-pager_con {
      width: calc(2 * var(--theme-page-item-size));
      height: calc(2 * var(--theme-page-item-size));
      position: absolute;
      left: 0px;
      top: 0px;
      z-index: 2;
    }
    
    .bk-pager_box {
      width: 100%;
      height: 100%;
      font-size: 0px;
      position: relative;
    }
    
    .bk-pager_item-text {
      display: block;
      width: 100%;
      height: 100%;
      font-family: ZagRegular;
      line-height: var(--theme-page-item-size);
      color: var(--theme-base-color);
      letter-spacing: 2px;
    }
    
    .bk-pager_item {
      position: absolute;
      color: white;
      font-size: 20px;
      width: var(--theme-page-item-size);
      height: var(--theme-page-item-size);
      left: var(--theme-page-item-size);
      top: var(--theme-page-item-size);
      line-height: 40px;
      text-align: center;
      background-color: var(--theme-page-nav-bg-color);
      cursor: pointer;
      transition: all 0.2s ease;
    }
    
    .bk-pager_item:hover,.bk-pager_item.current {
      background-color: var(--theme-page-item-hover-color);
    }
    
    .bk-pager_item.current {
      cursor: not-allowed;
      pointer-events: none;
    }
    
    .bk-pager_item.middle {
      font-size: 14px;
    }
    
    .bk-pager_go {
      width: 2 * var(--theme-page-item-size);
      height: 2 * var(--theme-page-item-size);
      position: absolute;
      font-size: 0px;
      left: var(--theme-page-item-size);
      top: var(--theme-page-item-size);
      z-index: 1;
    }
    
    .bk-pager_go-span,
    .bk-pager_go-input {
      text-align: center;
      font-size: 20px;
      font-family: ZagRegular;
      display: block;
      color: white;
      width: 100%;
      height: 100%;
      text-transform: uppercase;
      letter-spacing: 2px;
      border: none;
      background: none;
    }
    
    .bk-pager_go-span::placeholder {
      color: var(--theme-base-color);
    }
    
    .bk-pager_go-input::placeholder {
      color: var(--theme-base-color);
    }
    
    .bk-pager_go-span {
      font-size: 12px;
      line-height: var(--theme-page-item-size);
      text-overflow: ellipsis;
      overflow: hidden;
      color: var(--theme-base-color);
      white-space: nowrap;
    }
    
    .bk-pager_go-input {
      color: white;
      font-size: 14px;
      outline: none;
      box-sizing: border-box;
      border-bottom: 4px solid var(--theme-base-color);
      color: var(--theme-base-color);
    }
    
    .bk-pager_go-input.animation {
      animation: blink 1s ease-in-out infinite alternate-reverse both;
    }
    
    .bk-pager_go-em,
    .bk-pager_go-div,
    .bk-pager_btn {
      width: var(--theme-page-item-size);
      height: var(--theme-page-item-size);
      box-sizing: border-box;
      display: inline-block;
      position: absolute;
      margin: auto;
      border: none;
      left: 0;
      top: 0;
      vertical-align: top;
      color: var(--theme-base-color);
      background-color: var(--theme-page-nav-bg-color);
      transition: all 0.2s ease;
    }
    
    .bk-pager_go-em {
      background: none;
    }
    
    .bk-pager_btn {
      margin: 0;
      padding: 0;
      cursor: pointer;
    }
    
    .bk-pager_btn:hover {
      background-color: var(--theme-page-item-hover-color);
    }
    
    .bk-pager_btn span {
      width: 40%;
      height: 40%;
      display: block;
      background: var(--theme-base-color);
      position: absolute;
      right: 0;
      bottom: 0;
    }
    
    @keyframes breathPage {
      0% {
        background-color: var(--theme-base-color);
      }
      100% {
        background-color: var(--theme-page-item-breath-color2);
      }
    }
    
    @keyframes breathPage1 {
      0% {
        background-color: var(--theme-page-item-breath-color1);
      }
      50% {
        background-color: var(--theme-page-item-breath-color2);
      }
      100% {
        background-color: var(--theme-base-color);
      }
    }
    
    @keyframes breathPage2 {
      0% {
        background-color: var(--theme-page-item-breath-color2);
      }
      100% {
        background-color: var(--theme-base-color);
      }
    }
    
    @keyframes blink {
      0% {
        border-color: transparent;
      }
      100% {
        border-color: var(--theme-base-color);
      }
    }
    "#
  ))
}
