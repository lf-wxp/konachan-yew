use crate::store::Image;

pub(crate) struct UseWaterfallProps {
  security: bool,
  width: f64,
  images: Vec<Image>,
  max_width: f64,
  min_width: f64,
}

fn shortest_column_pure(col_array: &[f64]) -> (f64, usize) {
  if col_array.is_empty() {
    return (0.0, 0);
  }

  let mut min = f64::MAX;
  let mut index = 0;

  for (i, &height) in col_array.iter().enumerate() {
    if height < min {
      min = height;
      index = i;
    }
  }

  (min, index)
}

fn calc_column_width(mw: f64, mm: f64, w: f64) -> (usize, f64) {
  if w > 0.0 {
    let col_width = if w % mw > 0.0 {
      w / (w / mw).ceil()
    } else {
      mw
    };

    if col_width < mm {
      (1, w)
    } else {
      ((w / mw).ceil() as usize, col_width)
    }
  } else {
    (0, 0.0)
  }
}

fn calc_column_array_pure(h: f64, col_array: &mut Vec<f64>) {
  if !col_array.is_empty() {
    let (_min, index) = shortest_column_pure(col_array);
    col_array[index] += h;
  }
}

fn calc_position_pure(col_width: f64, col_array: &[f64]) -> (f64, f64) {
  let (min, index) = shortest_column_pure(col_array);
  (index as f64 * col_width, min)
}

fn calc_list_item_size(item: &Image, col_width: f64, cols: &[f64]) -> Image {
  let ratio = item.height / item.width;
  let height = col_width * ratio as f64;
  let (x, y) = calc_position_pure(col_width, cols);
  Image {
    style_w: Some(col_width),
    style_h: Some(height),
    style: Some(format!(
      "width: {}px; height: {}px; transform: translateX({}px) translateY({}px)",
      col_width, height, x, y
    )),
    ..item.clone()
  }
}

fn update_layout_pure(
  security: bool,
  col_width: f64,
  col_array: &mut Vec<f64>,
  images: &[Image],
) -> Vec<Image> {
  let mut items = Vec::new();
  for item in images.iter().filter(|&x| {
    if security {
      return x.security;
    }
    true
  }) {
    let new_item = calc_list_item_size(item, col_width, col_array);
    calc_column_array_pure(new_item.style_h.unwrap_or(0.0), col_array);
    items.push(new_item);
  }

  items
}

pub(crate) struct WaterfallParams {
  pub(crate) security: bool,
  pub(crate) width: f64,
  pub(crate) max_width: f64,
  pub(crate) min_width: f64,
  pub(crate) images: Vec<Image>,
}

pub(crate) fn calc_waterfall(params: WaterfallParams) -> Vec<Image> {
  let WaterfallParams {
    security,
    width,
    max_width,
    min_width,
    images,
  } = params;
  let (column, col_width) = calc_column_width(max_width, min_width, width);
  let mut col_array = vec![0.0 as f64; column];
  update_layout_pure(security, col_width, &mut col_array, &images)
}
