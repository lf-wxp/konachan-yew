//! Konachan - A modern web application for browsing and downloading images
//!
//! This is a Tauri and Web-based application that provides a beautiful interface
//! for browsing Konachan image galleries with advanced features like:
//! - Waterfall layout display
//! - Download management
//! - Dynamic wallpapers
//! - Multi-language support (English and Chinese)

use stylist::{self, style};
use yew::prelude::*;

use components::{
  Background, DownloadList, DynamicWallpaper, List, Loader, Nav, NotifyProvider, Search, Service,
  Setting,
};
#[cfg(not(feature = "tauri"))]
use utils::register_ws;
use utils::{I18nProvider, TRANSLATIONS, style};

use crate::store::BounceRoot;

mod components;
mod hook;
mod model;
mod store;
mod utils;

#[function_component]
fn App() -> Html {
  let class_name = get_class_name();
  let supported_languages = vec!["en", "zh"];
  #[cfg(not(feature = "tauri"))]
  {
    register_ws();
  }
  html! {
      <BounceRoot>
        <NotifyProvider>
          <I18nProvider supported_languages={supported_languages} translations={TRANSLATIONS.clone()} >
            <Service />
            <section class={class_name}>
              <Background />
              <DynamicWallpaper />
              <div class={"side"}>
                <Nav />
                <Setting />
                <Search />
                <DownloadList />
              </div>
              <div class="content">
                <List />
                <Loader />
              </div>
            </section>
            </I18nProvider>
        </NotifyProvider>
      </BounceRoot>
  }
}

#[allow(non_upper_case_globals)]
fn get_class_name() -> String {
  style::get_class_name(style!(
    r#"
      block-size: 100vh;
      overflow: hidden;
      display: grid;
      grid-template-columns: 120px auto;
      grid-template-rows: 100%;
      grid-auto-flow: column;

      .side {
        position: relative;
        background: color(var(--theme-base-color) alpha(20%));
        backdrop-filter: blur(5px);
        padding: 0 6px;
        display: flex;
        flex-flow: column nowrap;
      }
      .side > section  {
        margin-block-end: 10px;
      }
      
      .content {
        position: relative;
      }
    "#
  ))
}

fn main() {
  yew::Renderer::<App>::new().render();
}
