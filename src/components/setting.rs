use bounce::{use_atom, use_atom_value};
use stylist::{self, style};
use yew::prelude::*;

use crate::{
  store::{Invertible, Loading, Mode, Refresh, Security},
  utils::style,
};

#[function_component]
pub fn Setting() -> Html {
  let class_name = get_class_name();
  let security = use_atom::<Security>();
  let loading = use_atom_value::<Loading>();
  let refresh = use_atom::<Refresh>();
  let mode = use_atom::<Mode>();

  let active_class = format!(
    "bk-setting__security animation {}",
    if *security.value() { "active" } else { "" }
  );
  let loading_class = format!(
    "bk-setting__refresh animation {}",
    if *loading.value() { "active" } else { "" }
  );
  let mode_class = format!("bk-mode {}", *mode);

  let security_click = Callback::from(move |_: MouseEvent| {
    security.set(security.invert());
  });

  let mode_click = Callback::from(move |_: MouseEvent| {
    mode.set(mode.invert());
  });

  let refresh_click = Callback::from(move |_: MouseEvent| {
    refresh.set(refresh.invert());
  });

  let mut is_tauri = false;
  let mut is_safe = false;

  #[cfg(feature = "tauri")]
  {
    is_tauri = true;
  }
  #[cfg(feature = "safe")]
  {
    is_safe = true;
  }

  html! {
    <section class={class_name}>
      if !is_safe {
        <article class={active_class}>
          <label class="bk-setting__toggle" onclick={security_click}>
            <span class="bk-setting__fake animation" />
          </label>
        </article>
      }
      if is_tauri {
        <article
          class={mode_class}
          onclick={mode_click}
        >
          <span class="bk-mode__toggle animation"/>
        </article>
      }
      <article
        class={loading_class}
        onclick={refresh_click}
      >
        <div />
        <div />
        <div />
        <div />
        <div />
      </article>
    </section>
  }
}

#[allow(non_upper_case_globals)]
fn get_class_name() -> String {
  style::get_class_name(style!(
    r#"
      display: flex;
      justify-content: space-between;
      align-items: stretch;
      .bk-setting__security {
        flex: 0 0 auto;
        width: 32px;
        height: 32px;
        position: relative;
        border-right: 4px solid var(--theme-base-color);
        border-bottom: 4px solid var(--theme-base-color);
        border-top: none;
        border-left: none;
        transition: all 0.2s ease;
      }
    
      .bk-setting__security.animation {
        animation: breathSet 2s ease-in-out 0.5s infinite alternate-reverse both;
      }
    
      .bk-setting__security.active {
        border-top: 4px solid var(--theme-base-color);
        border-left: 4px solid var(--theme-base-color);
        border-bottom: none;
        border-right: none;
      }
    
      .bk-setting__security.active .bk-setting__fake {
        transform: translate(100%, 100%);
      }
    
      .bk-setting__toggle {
        display: block;
        height: 100%;
        cursor: pointer;
      }
      
      .bk-setting__fake {
        position: absolute;
        width: 50%;
        height: 50%;
        transition: transform 0.2s ease;
        background: var(--theme-base-color);
      }
      
      .bk-setting__fake.animation {
        animation: breathItem 2s ease-in-out infinite alternate-reverse both;
      }
      
      .bk-setting__refresh {
        flex: 0 0 auto;
        display: flex;
        justify-content: center;
        align-items: center;
        width: 32px;
        height: 32px;
        position: relative;
        transition: all 0.1s linear;
        cursor: pointer;
        border-top: 4px solid var(--theme-base-color);
        border-bottom: 4px solid var(--theme-base-color);
        border-right: 0;
        border-left: 0;
        box-sizing: border-box;
      }
      
      .bk-setting__refresh.animation {
        animation: breathSet 2s ease-in-out infinite alternate-reverse both;
      }
      
      .bk-setting__refresh > div {
        height: 100%;
        width: 6px;
        display: inline-block;
      }
      
      .bk-setting__refresh.active {
        border-width: 0;
        cursor: default;
      }
      
      .bk-setting__refresh.active > div {
        background-color: var(--theme-base-color);
        animation: sk-stretchdelay 1.2s infinite ease-in-out;
      }
      
      .bk-setting__refresh.active > div:nth-child(2) {
        animation-delay: -1.1s;
      }
      
      .bk-setting__refresh.active > div:nth-child(3) {
        animation-delay: -1s;
      }

      .bk-setting__refresh.active > div:nth-child(4) {
        animation-delay: -0.9s;
      }
      
      .bk-setting__refresh.active > div:nth-child(5) {
        animation-delay: -0.8s;
      }

      .bk-mode {
        flex: 0 0 auto;
        width: 32px;
        height: 32px;
        position: relative;
        cursor: pointer;
      }
      
      .bk-mode.json .bk-mode__toggle {
        transform: translateY(28px);
      }
      
      .bk-mode__toggle {
        display: block;
        cursor: pointer;
        height: 4px;
        transition: transform 0.2s ease;
        background: var(--theme-base-color);
      }
      
      .bk-mode__toggle.animation {
        animation: breathItem 2s ease-in-out 0.5s infinite alternate-reverse both;
      }
      
      @keyframes sk-stretchdelay {
        0%,
        40%,
        100% {
          transform: scaleY(0.4);
        }
        20% {
          transform: scaleY(1);
        }
      }
      
      @keyframes breathSet {
        0% {
          border-color: transparent;
        }
        100% {
          border-color: var(--theme-base-color);
        }
      }
      
      @keyframes breathItem {
        0% {
          background: transparent;
        }
        100% {
          background: var(--theme-base-color);
        }
      }
    "#
  ))
}
