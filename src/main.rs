use bounce::BounceRoot;
use stylist::{self, style};
use yew::prelude::*;

use components::{Background, DynamicWallpaper, List, Loader, Nav, Service, Setting, Search};
use utils::style;

mod components;
mod hook;
mod model;
mod store;
mod utils;

#[function_component]
fn App() -> Html {
  let class_name = get_class_name();
  html! {
      <BounceRoot>
        <Service />
        <section class={class_name}>
          <Background />
          <DynamicWallpaper />
          <div class={"side"}>
            <Nav />
            <Setting />
            <Search />
          </div>
          <div class="content">
            <List />
            <Loader />
          </div>
        </section>
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
