#[cfg(test)]
mod measure {
    use stretch2::node::MeasureFunc;
    use stretch2::number::OrElse;

    #[test]
    fn measure_root() {
        let mut stretch = stretch2::node::Stretch::new();
        let node = stretch
            .new_leaf(
                stretch2::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| stretch2::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();

        assert_eq!(stretch.layout(node).unwrap().size.width, 100.0);
        assert_eq!(stretch.layout(node).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child() {
        let mut stretch = stretch2::node::Stretch::new();

        let child = stretch
            .new_leaf(
                stretch2::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| stretch2::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = stretch.new_node(stretch2::style::Style { ..Default::default() }, &[child]).unwrap();
        stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();

        assert_eq!(stretch.layout(node).unwrap().size.width, 100.0);
        assert_eq!(stretch.layout(node).unwrap().size.height, 100.0);

        assert_eq!(stretch.layout(child).unwrap().size.width, 100.0);
        assert_eq!(stretch.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint() {
        let mut stretch = stretch2::node::Stretch::new();
        let child = stretch
            .new_leaf(
                stretch2::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| stretch2::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = stretch
            .new_node(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Points(50.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();

        assert_eq!(stretch.layout(node).unwrap().size.width, 50.0);
        assert_eq!(stretch.layout(node).unwrap().size.height, 100.0);

        assert_eq!(stretch.layout(child).unwrap().size.width, 50.0);
        assert_eq!(stretch.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint_padding_parent() {
        let mut stretch = stretch2::node::Stretch::new();
        let child = stretch
            .new_leaf(
                stretch2::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| stretch2::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = stretch
            .new_node(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Points(50.0),
                        ..Default::default()
                    },
                    padding: stretch2::geometry::Rect {
                        start: stretch2::style::Dimension::Points(10.0),
                        end: stretch2::style::Dimension::Points(10.0),
                        top: stretch2::style::Dimension::Points(10.0),
                        bottom: stretch2::style::Dimension::Points(10.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();
        stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();

        assert_eq!(stretch.layout(node).unwrap().size.width, 50.0);
        assert_eq!(stretch.layout(node).unwrap().size.height, 120.0);

        assert_eq!(stretch.layout(child).unwrap().size.width, 30.0);
        assert_eq!(stretch.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_with_flex_grow() {
        let mut stretch = stretch2::node::Stretch::new();
        let child0 = stretch
            .new_node(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Points(50.0),
                        height: stretch2::style::Dimension::Points(50.0),
                    },
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        let child1 = stretch
            .new_leaf(
                stretch2::style::Style { flex_grow: 1.0, ..Default::default() },
                MeasureFunc::Raw(|constraint| stretch2::geometry::Size {
                    width: constraint.width.or_else(10.0),
                    height: constraint.height.or_else(50.0),
                }),
            )
            .unwrap();

        let node = stretch
            .new_node(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Points(100.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();

        assert_eq!(stretch.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(stretch.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn measure_child_with_flex_shrink() {
        let mut stretch = stretch2::node::Stretch::new();
        let child0 = stretch
            .new_node(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Points(50.0),
                        height: stretch2::style::Dimension::Points(50.0),
                    },
                    flex_shrink: 0.0,
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        let child1 = stretch
            .new_leaf(
                stretch2::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| stretch2::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(50.0),
                }),
            )
            .unwrap();

        let node = stretch
            .new_node(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Points(100.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();

        assert_eq!(stretch.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(stretch.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn remeasure_child_after_growing() {
        let mut stretch = stretch2::node::Stretch::new();
        let child0 = stretch
            .new_node(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Points(50.0),
                        height: stretch2::style::Dimension::Points(50.0),
                    },
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        let child1 = stretch
            .new_leaf(
                stretch2::style::Style { flex_grow: 1.0, ..Default::default() },
                MeasureFunc::Raw(|constraint| {
                    let width = constraint.width.or_else(10.0);
                    let height = constraint.height.or_else(width * 2.0);
                    stretch2::geometry::Size { width, height }
                }),
            )
            .unwrap();

        let node = stretch
            .new_node(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Points(100.0),
                        ..Default::default()
                    },
                    align_items: stretch2::style::AlignItems::FlexStart,
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();

        assert_eq!(stretch.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(stretch.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn remeasure_child_after_shrinking() {
        let mut stretch = stretch2::node::Stretch::new();

        let child0 = stretch
            .new_node(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Points(50.0),
                        height: stretch2::style::Dimension::Points(50.0),
                    },
                    flex_shrink: 0.0,
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        let child1 = stretch
            .new_leaf(
                stretch2::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| {
                    let width = constraint.width.or_else(100.0);
                    let height = constraint.height.or_else(width * 2.0);
                    stretch2::geometry::Size { width, height }
                }),
            )
            .unwrap();

        let node = stretch
            .new_node(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Points(100.0),
                        ..Default::default()
                    },
                    align_items: stretch2::style::AlignItems::FlexStart,
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();

        assert_eq!(stretch.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(stretch.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn remeasure_child_after_stretching() {
        let mut stretch = stretch2::node::Stretch::new();

        let child = stretch
            .new_leaf(
                stretch2::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| {
                    let height = constraint.height.or_else(50.0);
                    let width = constraint.width.or_else(height);
                    stretch2::geometry::Size { width, height }
                }),
            )
            .unwrap();

        let node = stretch
            .new_node(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Points(100.0),
                        height: stretch2::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();

        assert_eq!(stretch.layout(child).unwrap().size.width, 100.0);
        assert_eq!(stretch.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn width_overrides_measure() {
        let mut stretch = stretch2::node::Stretch::new();
        let child = stretch
            .new_leaf(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Points(50.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                MeasureFunc::Raw(|constraint| stretch2::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = stretch.new_node(stretch2::style::Style { ..Default::default() }, &[child]).unwrap();
        stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();

        assert_eq!(stretch.layout(child).unwrap().size.width, 50.0);
        assert_eq!(stretch.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn height_overrides_measure() {
        let mut stretch = stretch2::node::Stretch::new();
        let child = stretch
            .new_leaf(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        height: stretch2::style::Dimension::Points(50.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                MeasureFunc::Raw(|constraint| stretch2::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = stretch.new_node(stretch2::style::Style { ..Default::default() }, &[child]).unwrap();
        stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();

        assert_eq!(stretch.layout(child).unwrap().size.width, 100.0);
        assert_eq!(stretch.layout(child).unwrap().size.height, 50.0);
    }

    #[test]
    fn flex_basis_overrides_measure() {
        let mut stretch = stretch2::node::Stretch::new();
        let child0 = stretch
            .new_node(
                stretch2::style::Style {
                    flex_basis: stretch2::style::Dimension::Points(50.0),
                    flex_grow: 1.0,
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        let child1 = stretch
            .new_leaf(
                stretch2::style::Style {
                    flex_basis: stretch2::style::Dimension::Points(50.0),
                    flex_grow: 1.0,
                    ..Default::default()
                },
                MeasureFunc::Raw(|constraint| stretch2::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = stretch
            .new_node(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Points(200.0),
                        height: stretch2::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();

        assert_eq!(stretch.layout(child0).unwrap().size.width, 100.0);
        assert_eq!(stretch.layout(child0).unwrap().size.height, 100.0);
        assert_eq!(stretch.layout(child1).unwrap().size.width, 100.0);
        assert_eq!(stretch.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn stretch_overrides_measure() {
        let mut stretch = stretch2::node::Stretch::new();
        let child = stretch
            .new_leaf(
                stretch2::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| stretch2::geometry::Size {
                    width: constraint.width.or_else(50.0),
                    height: constraint.height.or_else(50.0),
                }),
            )
            .unwrap();

        let node = stretch
            .new_node(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Points(100.0),
                        height: stretch2::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();

        assert_eq!(stretch.layout(child).unwrap().size.width, 50.0);
        assert_eq!(stretch.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_absolute_child() {
        let mut stretch = stretch2::node::Stretch::new();
        let child = stretch
            .new_leaf(
                stretch2::style::Style { position_type: stretch2::style::PositionType::Absolute, ..Default::default() },
                MeasureFunc::Raw(|constraint| stretch2::geometry::Size {
                    width: constraint.width.or_else(50.0),
                    height: constraint.height.or_else(50.0),
                }),
            )
            .unwrap();

        let node = stretch
            .new_node(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Points(100.0),
                        height: stretch2::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();

        assert_eq!(stretch.layout(child).unwrap().size.width, 50.0);
        assert_eq!(stretch.layout(child).unwrap().size.height, 50.0);
    }

    #[test]
    fn ignore_invalid_measure() {
        let mut stretch = stretch2::node::Stretch::new();
        let child = stretch
            .new_leaf(
                stretch2::style::Style { flex_grow: 1.0, ..Default::default() },
                MeasureFunc::Raw(|_| stretch2::geometry::Size { width: 200.0, height: 200.0 }),
            )
            .unwrap();

        let node = stretch
            .new_node(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Points(100.0),
                        height: stretch2::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();

        assert_eq!(stretch.layout(child).unwrap().size.width, 100.0);
        assert_eq!(stretch.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn only_measure_once() {
        use std::sync::atomic;

        let mut stretch = stretch2::node::Stretch::new();
        static NUM_MEASURES: atomic::AtomicU32 = atomic::AtomicU32::new(0);

        let grandchild = stretch
            .new_leaf(
                stretch2::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| {
                    NUM_MEASURES.fetch_add(1, atomic::Ordering::Relaxed);
                    stretch2::geometry::Size {
                        width: constraint.width.or_else(50.0),
                        height: constraint.height.or_else(50.0),
                    }
                }),
            )
            .unwrap();

        let child = stretch.new_node(stretch2::style::Style { ..Default::default() }, &[grandchild]).unwrap();

        let node = stretch.new_node(stretch2::style::Style { ..Default::default() }, &[child]).unwrap();
        stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();

        assert_eq!(NUM_MEASURES.load(atomic::Ordering::Relaxed), 2);
    }
}
