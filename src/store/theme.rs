use bounce::Atom;

#[derive(Atom, PartialEq, Debug)]
pub(crate) struct ThemeColor {
  pub theme_color: String,
  pub primary_color: String,
  pub ancillary_color: String,
}

impl ThemeColor {
  pub fn new(theme: String, primary: String, ancillary: String) -> ThemeColor {
    ThemeColor {
      theme_color: theme,
      primary_color: primary,
      ancillary_color: ancillary,
    }
  }
  pub fn get_css_text(&self) -> String {
    let ThemeColor {
      theme_color,
      primary_color,
      ancillary_color,
    } = self;
    format!("--theme-base-color: rgb({theme_color});--theme-base-color-rgb: {theme_color};--theme-primary-color: rgb({primary_color});--theme-ancillary-color: rgb({ancillary_color});")
  }
  pub fn get_color(&self) -> (String, String, String) {
    let ThemeColor {
      theme_color,
      primary_color,
      ancillary_color,
    } = self;
    (
      format!("rgb({theme_color})"),
      format!("rgb({primary_color})"),
      format!("rgb({ancillary_color})"),
    )
  }
}

impl Default for ThemeColor {
  fn default() -> Self {
    Self {
      theme_color: "57, 204, 204".to_string(),
      primary_color: Default::default(),
      ancillary_color: Default::default(),
    }
  }
}
