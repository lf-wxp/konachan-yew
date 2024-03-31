use bounce::{use_atom, use_atom_value};
use gloo_console::log;
use stylist::{self, style};
use web_sys::HtmlLiElement;
use yew::prelude::*;

use crate::{
  store::{Page, Total},
  utils::{get_target, style},
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
  let page_handle = use_atom::<Page>();
  let page = use_atom_value::<Page>();
  let total = use_atom_value::<Total>();
  let page_vec = get_page_vec(page.value(), total.value());

  let next = Callback::from(move |_: MouseEvent| {});
  let prev = Callback::from(move |_: MouseEvent| {});
  let invoke = Callback::from(move |id: u32| {
    log!("invoke", id);
    page_handle.set(Page::new(id));
  });
  let goto = Callback::from(move |_: MouseEvent| {});
  let change = Callback::from(move |_: Event| {});

  let bk_page = {
    let param = if !page_vec.is_empty() { "active" } else { "" };
    format!("bk-pager {}", param)
  };

  let bk_page_nav_prev = {
    let param = if page.value() - 1 > 0 { "" } else { "disabled" };
    format!("bk-pager_nav {}", param)
  };

  let bk_page_nav_next = {
    let param = if total.value() - page.value() > 0 {
      ""
    } else {
      "disabled"
    };
    format!("bk-pager_nav {}", param)
  };

  let bk_page_item = |item: &u32| -> String {
    let current = if page.value() == *item { "current" } else { "" };
    let size = if page.value() >= 102 { "middle" } else { "" };
    format!("bk-pager_item {} {}", current, size)
  };

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
            <span class="bk-pager_go-span">{total.value()}</span>
          </div>
          <div class="bk-pager_go-div">
            <input
              class={"bk-pager_go-input animation"}
              type="text"
              placeholder="page"
              name="pager"
              value={page.value().to_string()}
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
      width: calc(3 * var(--themePageItemSize));
      height: calc(3 * var(--themePageItemSize));
      position: relative;
      z-index: 2;
      font-family: 'NanoCore';
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
      left: var(--themePageItemSize);
      top: 0px;
    }
    
    .bk-pager.active .bk-pager_go .bk-pager_go-div:last-of-type {
      top: var(--themePageItemSize);
      left: 0px;
      transition-delay: 0.1s;
    }
    
    .bk-pager.active .bk-pager_go .bk-pager_btn {
      top: var(--themePageItemSize);
      left: var(--themePageItemSize);
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
      left: var(--themePageItemSize);
      top: 0px;
      transition-delay: 0.1s;
    }
    
    .bk-pager.active .bk-pager_item:nth-child(3) {
      left: 0px;
      top: var(--themePageItemSize);
      transition-delay: 0.2s;
    }
    
    .bk-pager_nav {
      width: var(--themePageItemSize);
      height: var(--themePageItemSize);
      position: absolute;
      display: inline-block;
      color: white;
      font-size: 30px;
      line-height: 40px;
      text-align: center;
      background-color: var(--themePageNavBgColor);
      cursor: pointer;
      transition: all 0.3s ease;
      bottom: var(--themePageItemSize);
      left: var(--themePageItemSize);
    }
    
    .bk-pager_nav:hover:after,.bk-pager_nav:hover:before {
      background: var(--themePageItemHoverColor) !important;
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
      background: var(--themePageItemHoverColor);
    }
    
    .bk-pager_nav:nth-of-type(1):before {
      content: '';
      position: absolute;
      left: 0;
      bottom: 0;
      height: 100%;
      width: 5px;
      background: var(--themePageItemHoverColor);
    }
    
    .bk-pager_nav:nth-of-type(2):after {
      content: '';
      position: absolute;
      right: 5px;
      top: 0;
      height: 5px;
      width: calc(100% - 5px);
      background: var(--themePageItemHoverColor);
    }

    .bk-pager_nav:nth-of-type(2):before {
      content: '';
      position: absolute;
      right: 0;
      top: 0;
      height: 100%;
      width: 5px;
      background: var(--themePageItemHoverColor);
    }
    
    .bk-pager_nav.disabled {
      pointer-events: none;
      cursor: not-allowed;
    }
    
    .bk-pager_holder {
      width: var(--themePageItemSize);
      height: var(--themePageItemSize);
      position: absolute;
      background-color: var(--themeBaseColor);
      z-index: 3;
      left: var(--themePageItemSize);
      top: var(--themePageItemSize);
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
      background-color: var(--themePageItemBreathColor2);
      animation: breathPage2 2s ease-in-out alternate infinite;
    }
    
    .bk-pager_holder:before {
      width: 70%;
      height: 70%;
      background-color: var(--themePageItemBreathColor1);
      animation: breathPage1 2s 2s ease-in-out alternate infinite;
    }
    
    .bk-pager_con {
      width: calc(2 * var(--themePageItemSize));
      height: calc(2 * var(--themePageItemSize));
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
      line-height: var(--themePageItemSize);
      color: var(--themeBaseColor);
      letter-spacing: 2px;
    }
    
    .bk-pager_item {
      position: absolute;
      color: white;
      font-size: 20px;
      width: var(--themePageItemSize);
      height: var(--themePageItemSize);
      left: var(--themePageItemSize);
      top: var(--themePageItemSize);
      line-height: 40px;
      text-align: center;
      background-color: var(--themePageNavBgColor);
      cursor: pointer;
      transition: all 0.2s ease;
    }
    
    .bk-pager_item:hover,.bk-pager_item.current {
      background-color: var(--themePageItemHoverColor);
    }
    
    .bk-pager_item.current {
      cursor: not-allowed;
      pointer-events: none;
    }
    
    .bk-pager_item.middle {
      font-size: 14px;
    }
    
    .bk-pager_go {
      width: 2 * var(--themePageItemSize);
      height: 2 * var(--themePageItemSize);
      position: absolute;
      font-size: 0px;
      left: var(--themePageItemSize);
      top: var(--themePageItemSize);
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
      color: var(--themeBaseColor);
    }
    
    .bk-pager_go-input::placeholder {
      color: var(--themeBaseColor);
    }
    
    .bk-pager_go-span {
      font-size: 12px;
      line-height: var(--themePageItemSize);
      text-overflow: ellipsis;
      overflow: hidden;
      color: var(--themeBaseColor);
      white-space: nowrap;
    }
    
    .bk-pager_go-input {
      color: white;
      font-size: 14px;
      outline: none;
      box-sizing: border-box;
      border-bottom: 4px solid var(--themeBaseColor);
      color: var(--themeBaseColor);
    }
    
    .bk-pager_go-input.animation {
      animation: blink 1s ease-in-out infinite alternate-reverse both;
    }
    
    .bk-pager_go-em,
    .bk-pager_go-div,
    .bk-pager_btn {
      width: var(--themePageItemSize);
      height: var(--themePageItemSize);
      box-sizing: border-box;
      display: inline-block;
      position: absolute;
      margin: auto;
      border: none;
      left: 0;
      top: 0;
      vertical-align: top;
      color: var(--themeBaseColor);
      background-color: var(--themePageNavBgColor);
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
      background-color: var(--themePageItemHoverColor);
    }
    
    .bk-pager_btn span {
      width: 40%;
      height: 40%;
      display: block;
      background: var(--themeBaseColor);
      position: absolute;
      right: 0;
      bottom: 0;
    }
    
    @keyframes breathPage {
      0% {
        background-color: var(--themeBaseColor);
      }
      100% {
        background-color: var(--themePageItemBreathColor2);
      }
    }
    
    @keyframes breathPage1 {
      0% {
        background-color: var(--themePageItemBreathColor1);
      }
      50% {
        background-color: var(--themePageItemBreathColor2);
      }
      100% {
        background-color: var(--themeBaseColor);
      }
    }
    
    @keyframes breathPage2 {
      0% {
        background-color: var(--themePageItemBreathColor2);
      }
      100% {
        background-color: var(--themeBaseColor);
      }
    }
    
    @keyframes blink {
      0% {
        border-color: transparent;
      }
      100% {
        border-color: var(--themeBaseColor);
      }
    }
    "#
  ))
}