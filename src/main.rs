use bounce::BounceRoot;
use stylist::{self, style};
use yew::prelude::*;

use components::{Background, Service, Nav, DynamicWallpaper, List};
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
            </div>
            <div class="content">
              <List />
            </div>
          </section>
      </BounceRoot>
  }
}

#[allow(non_upper_case_globals)]
fn get_class_name() -> String {
  style::get_class_name(style!(
    r#"
        display: flex;
        flex-flow: nowrap; 
        inline-size: 100%;
        block-size: 100%;
        --padding: 5px;
        .side {
          padding: var(--padding);
          backdrop-filter: blur(15px);
          z-index: 1;
        }
        
        .content {
          padding: calc(var(--padding) * 2);
          flex: 1 1 auto;
        }
    "#
  ))
}

fn main() {
  // set_client();
  yew::Renderer::<App>::new().render();
}
