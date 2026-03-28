use crate::store::Image;

/// Find the shortest column and return its (height, index).
fn shortest_column(col_array: &[f64]) -> (f64, usize) {
  if col_array.is_empty() {
    return (0.0, 0);
  }

  col_array
    .iter()
    .enumerate()
    .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    .map(|(i, &h)| (h, i))
    .unwrap_or((0.0, 0))
}

/// Calculate column count and width from container width and constraints.
fn calc_column_width(max_width: f64, min_width: f64, width: f64) -> (usize, f64) {
  if width <= 0.0 {
    return (0, 0.0);
  }

  let col_width = if width % max_width > 0.0 {
    width / (width / max_width).ceil()
  } else {
    max_width
  };

  if col_width < min_width {
    (1, width)
  } else {
    ((width / max_width).ceil() as usize, col_width)
  }
}

/// Add height to the shortest column.
fn add_to_shortest_column(height: f64, col_array: &mut [f64]) {
  if !col_array.is_empty() {
    let (_min, index) = shortest_column(col_array);
    col_array[index] += height;
  }
}

/// Calculate position (x, y) for the next item based on column state.
fn calc_position(col_width: f64, col_array: &[f64]) -> (f64, f64) {
  let (min, index) = shortest_column(col_array);
  (index as f64 * col_width, min)
}

/// Compute the styled Image for a single item given column state.
fn calc_list_item_size(item: &Image, col_width: f64, cols: &[f64]) -> Image {
  let ratio = item.height / item.width;
  let height = col_width * ratio;
  let (x, y) = calc_position(col_width, cols);
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

/// Lay out images into columns, filtering by security flag.
fn update_layout(
  security: bool,
  col_width: f64,
  col_array: &mut [f64],
  images: &[Image],
) -> Vec<Image> {
  let filtered_count = images.iter().filter(|x| !security || x.security).count();
  let mut items = Vec::with_capacity(filtered_count);

  for item in images.iter().filter(|x| !security || x.security) {
    let new_item = calc_list_item_size(item, col_width, col_array);
    add_to_shortest_column(new_item.style_h.unwrap_or(0.0), col_array);
    items.push(new_item);
  }

  items
}

/// Parameters for waterfall layout calculation.
pub(crate) struct WaterfallParams {
  pub(crate) security: bool,
  pub(crate) width: f64,
  pub(crate) max_width: f64,
  pub(crate) min_width: f64,
  pub(crate) images: Vec<Image>,
}

/// Calculate waterfall layout positions for all images.
pub fn calc_waterfall(params: WaterfallParams) -> Vec<Image> {
  let WaterfallParams {
    security,
    width,
    max_width,
    min_width,
    images,
  } = params;
  let (column, col_width) = calc_column_width(max_width, min_width, width);
  if column == 0 {
    return Vec::new();
  }
  let mut col_array = vec![0.0_f64; column];
  update_layout(security, col_width, &mut col_array, &images)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn make_image(id: u32, width: f64, height: f64, security: bool) -> Image {
    Image {
      id,
      width,
      height,
      security,
      sample_width: width,
      sample_height: height,
      preview_width: width,
      preview_height: height,
      sample: String::new(),
      preview: String::new(),
      url: String::new(),
      name: format!("img_{id}"),
      tags: None,
      style_h: None,
      style_w: None,
      style: None,
      full: None,
    }
  }

  // -- shortest_column --

  #[test]
  fn test_shortest_column_empty() {
    let (h, i) = shortest_column(&[]);
    assert_eq!(h, 0.0);
    assert_eq!(i, 0);
  }

  #[test]
  fn test_shortest_column_single() {
    let (h, i) = shortest_column(&[100.0]);
    assert_eq!(h, 100.0);
    assert_eq!(i, 0);
  }

  #[test]
  fn test_shortest_column_multiple() {
    let (h, i) = shortest_column(&[300.0, 100.0, 200.0]);
    assert_eq!(h, 100.0);
    assert_eq!(i, 1);
  }

  // -- calc_column_width --

  #[test]
  fn test_column_width_zero_width() {
    let (cols, w) = calc_column_width(300.0, 200.0, 0.0);
    assert_eq!(cols, 0);
    assert_eq!(w, 0.0);
  }

  #[test]
  fn test_column_width_negative_width() {
    let (cols, w) = calc_column_width(300.0, 200.0, -100.0);
    assert_eq!(cols, 0);
    assert_eq!(w, 0.0);
  }

  #[test]
  fn test_column_width_exact_multiple() {
    // 600px / 300px = 2 columns, each 300px
    let (cols, w) = calc_column_width(300.0, 200.0, 600.0);
    assert_eq!(cols, 2);
    assert_eq!(w, 300.0);
  }

  #[test]
  fn test_column_width_narrow_container() {
    // Container narrower than min_width -> 1 column, full width
    let (cols, w) = calc_column_width(300.0, 200.0, 150.0);
    assert_eq!(cols, 1);
    assert_eq!(w, 150.0);
  }

  // -- calc_waterfall --

  #[test]
  fn test_waterfall_empty_images() {
    let result = calc_waterfall(WaterfallParams {
      security: false,
      width: 600.0,
      max_width: 300.0,
      min_width: 200.0,
      images: vec![],
    });
    assert!(result.is_empty());
  }

  #[test]
  fn test_waterfall_zero_width() {
    let images = vec![make_image(1, 100.0, 200.0, true)];
    let result = calc_waterfall(WaterfallParams {
      security: false,
      width: 0.0,
      max_width: 300.0,
      min_width: 200.0,
      images,
    });
    assert!(result.is_empty());
  }

  #[test]
  fn test_waterfall_single_image() {
    let images = vec![make_image(1, 100.0, 200.0, true)];
    let result = calc_waterfall(WaterfallParams {
      security: false,
      width: 300.0,
      max_width: 300.0,
      min_width: 200.0,
      images,
    });
    assert_eq!(result.len(), 1);
    assert!(result[0].style_w.is_some());
    assert!(result[0].style_h.is_some());
    assert!(result[0].style.is_some());
  }

  #[test]
  fn test_waterfall_preserves_aspect_ratio() {
    let images = vec![make_image(1, 100.0, 200.0, true)];
    let result = calc_waterfall(WaterfallParams {
      security: false,
      width: 300.0,
      max_width: 300.0,
      min_width: 200.0,
      images,
    });
    let item = &result[0];
    let col_width = item.style_w.unwrap();
    let height = item.style_h.unwrap();
    // original ratio = 200/100 = 2.0
    let ratio = height / col_width;
    assert!((ratio - 2.0).abs() < 0.001);
  }

  #[test]
  fn test_waterfall_security_filter() {
    let images = vec![
      make_image(1, 100.0, 100.0, true),
      make_image(2, 100.0, 100.0, false),
      make_image(3, 100.0, 100.0, true),
    ];

    // security=true -> only safe images
    let result = calc_waterfall(WaterfallParams {
      security: true,
      width: 600.0,
      max_width: 300.0,
      min_width: 200.0,
      images: images.clone(),
    });
    assert_eq!(result.len(), 2);
    assert!(result.iter().all(|img| img.security));

    // security=false -> all images
    let result = calc_waterfall(WaterfallParams {
      security: false,
      width: 600.0,
      max_width: 300.0,
      min_width: 200.0,
      images,
    });
    assert_eq!(result.len(), 3);
  }

  #[test]
  fn test_waterfall_multi_column_distribution() {
    // 600px width, 300px max -> 2 columns
    let images: Vec<Image> = (0..4).map(|i| make_image(i, 100.0, 100.0, true)).collect();

    let result = calc_waterfall(WaterfallParams {
      security: false,
      width: 600.0,
      max_width: 300.0,
      min_width: 200.0,
      images,
    });

    assert_eq!(result.len(), 4);
    // All items should have style set
    for item in &result {
      assert!(item.style.is_some());
      assert!(item.style_w.is_some());
      assert!(item.style_h.is_some());
    }
  }
}
