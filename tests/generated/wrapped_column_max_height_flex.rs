#[test]
fn wrapped_column_max_height_flex() {
    let mut stretch = stretch2::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                flex_grow: 1f32,
                flex_shrink: 1f32,
                flex_basis: stretch2::style::Dimension::Percent(0f32),
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(100f32),
                    height: stretch2::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                max_size: stretch2::geometry::Size {
                    height: stretch2::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch2::style::Style {
                flex_grow: 1f32,
                flex_shrink: 1f32,
                flex_basis: stretch2::style::Dimension::Percent(0f32),
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(200f32),
                    height: stretch2::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                margin: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Points(20f32),
                    end: stretch2::style::Dimension::Points(20f32),
                    top: stretch2::style::Dimension::Points(20f32),
                    bottom: stretch2::style::Dimension::Points(20f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(100f32),
                    height: stretch2::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                flex_direction: stretch2::style::FlexDirection::Column,
                flex_wrap: stretch2::style::FlexWrap::Wrap,
                align_items: stretch2::style::AlignItems::Center,
                align_content: stretch2::style::AlignContent::Center,
                justify_content: stretch2::style::JustifyContent::Center,
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(700f32),
                    height: stretch2::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 700f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 500f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 180f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 300f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 200f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 180f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 250f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 200f32);
    assert_eq!(stretch.layout(node2).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node2).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node2).unwrap().location.x, 300f32);
    assert_eq!(stretch.layout(node2).unwrap().location.y, 400f32);
}
