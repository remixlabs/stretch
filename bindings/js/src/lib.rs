#![allow(non_snake_case)]

mod utils;

use std::cell::RefCell;
use std::rc::Rc;

use js_sys::Function;
use js_sys::Reflect;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl Into<stretch2::style::AlignItems> for AlignItems {
    fn into(self) -> stretch2::style::AlignItems {
        match self {
            AlignItems::FlexStart => stretch2::style::AlignItems::FlexStart,
            AlignItems::FlexEnd => stretch2::style::AlignItems::FlexEnd,
            AlignItems::Center => stretch2::style::AlignItems::Center,
            AlignItems::Baseline => stretch2::style::AlignItems::Baseline,
            AlignItems::Stretch => stretch2::style::AlignItems::Stretch,
        }
    }
}

impl From<i32> for AlignItems {
    fn from(n: i32) -> Self {
        match n {
            0 => AlignItems::FlexStart,
            1 => AlignItems::FlexEnd,
            2 => AlignItems::Center,
            3 => AlignItems::Baseline,
            4 => AlignItems::Stretch,
            _ => AlignItems::Stretch,
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignSelf {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl Into<stretch2::style::AlignSelf> for AlignSelf {
    fn into(self) -> stretch2::style::AlignSelf {
        match self {
            AlignSelf::Auto => stretch2::style::AlignSelf::Auto,
            AlignSelf::FlexStart => stretch2::style::AlignSelf::FlexStart,
            AlignSelf::FlexEnd => stretch2::style::AlignSelf::FlexEnd,
            AlignSelf::Center => stretch2::style::AlignSelf::Center,
            AlignSelf::Baseline => stretch2::style::AlignSelf::Baseline,
            AlignSelf::Stretch => stretch2::style::AlignSelf::Stretch,
        }
    }
}

impl From<i32> for AlignSelf {
    fn from(n: i32) -> Self {
        match n {
            0 => AlignSelf::Auto,
            1 => AlignSelf::FlexStart,
            2 => AlignSelf::FlexEnd,
            3 => AlignSelf::Center,
            4 => AlignSelf::Baseline,
            5 => AlignSelf::Stretch,
            _ => AlignSelf::Auto,
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

impl Into<stretch2::style::AlignContent> for AlignContent {
    fn into(self) -> stretch2::style::AlignContent {
        match self {
            AlignContent::FlexStart => stretch2::style::AlignContent::FlexStart,
            AlignContent::FlexEnd => stretch2::style::AlignContent::FlexEnd,
            AlignContent::Center => stretch2::style::AlignContent::Center,
            AlignContent::Stretch => stretch2::style::AlignContent::Stretch,
            AlignContent::SpaceBetween => stretch2::style::AlignContent::SpaceBetween,
            AlignContent::SpaceAround => stretch2::style::AlignContent::SpaceAround,
        }
    }
}

impl From<i32> for AlignContent {
    fn from(n: i32) -> Self {
        match n {
            0 => AlignContent::FlexStart,
            1 => AlignContent::FlexEnd,
            2 => AlignContent::Center,
            3 => AlignContent::Stretch,
            4 => AlignContent::SpaceBetween,
            5 => AlignContent::SpaceAround,
            _ => AlignContent::Stretch,
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Inherit,
    LTR,
    RTL,
}

impl Into<stretch2::style::Direction> for Direction {
    fn into(self) -> stretch2::style::Direction {
        match self {
            Direction::Inherit => stretch2::style::Direction::Inherit,
            Direction::LTR => stretch2::style::Direction::LTR,
            Direction::RTL => stretch2::style::Direction::RTL,
        }
    }
}

impl From<i32> for Direction {
    fn from(n: i32) -> Self {
        match n {
            0 => Direction::Inherit,
            1 => Direction::LTR,
            2 => Direction::RTL,
            _ => Direction::Inherit,
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Display {
    Flex,
    None,
}

impl Into<stretch2::style::Display> for Display {
    fn into(self) -> stretch2::style::Display {
        match self {
            Display::Flex => stretch2::style::Display::Flex,
            Display::None => stretch2::style::Display::None,
        }
    }
}

impl From<i32> for Display {
    fn from(n: i32) -> Self {
        match n {
            0 => Display::Flex,
            1 => Display::None,
            _ => Display::Flex,
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl Into<stretch2::style::FlexDirection> for FlexDirection {
    fn into(self) -> stretch2::style::FlexDirection {
        match self {
            FlexDirection::Row => stretch2::style::FlexDirection::Row,
            FlexDirection::Column => stretch2::style::FlexDirection::Column,
            FlexDirection::RowReverse => stretch2::style::FlexDirection::RowReverse,
            FlexDirection::ColumnReverse => stretch2::style::FlexDirection::ColumnReverse,
        }
    }
}

impl From<i32> for FlexDirection {
    fn from(n: i32) -> Self {
        match n {
            0 => FlexDirection::Row,
            1 => FlexDirection::Column,
            2 => FlexDirection::RowReverse,
            3 => FlexDirection::ColumnReverse,
            _ => FlexDirection::Row,
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Into<stretch2::style::JustifyContent> for JustifyContent {
    fn into(self) -> stretch2::style::JustifyContent {
        match self {
            JustifyContent::FlexStart => stretch2::style::JustifyContent::FlexStart,
            JustifyContent::FlexEnd => stretch2::style::JustifyContent::FlexEnd,
            JustifyContent::Center => stretch2::style::JustifyContent::Center,
            JustifyContent::SpaceBetween => stretch2::style::JustifyContent::SpaceBetween,
            JustifyContent::SpaceAround => stretch2::style::JustifyContent::SpaceAround,
            JustifyContent::SpaceEvenly => stretch2::style::JustifyContent::SpaceEvenly,
        }
    }
}

impl From<i32> for JustifyContent {
    fn from(n: i32) -> Self {
        match n {
            0 => JustifyContent::FlexStart,
            1 => JustifyContent::FlexEnd,
            2 => JustifyContent::Center,
            3 => JustifyContent::SpaceBetween,
            4 => JustifyContent::SpaceAround,
            5 => JustifyContent::SpaceEvenly,
            _ => JustifyContent::FlexStart,
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

impl Into<stretch2::style::Overflow> for Overflow {
    fn into(self) -> stretch2::style::Overflow {
        match self {
            Overflow::Visible => stretch2::style::Overflow::Visible,
            Overflow::Hidden => stretch2::style::Overflow::Hidden,
            Overflow::Scroll => stretch2::style::Overflow::Scroll,
        }
    }
}

impl From<i32> for Overflow {
    fn from(n: i32) -> Self {
        match n {
            0 => Overflow::Visible,
            1 => Overflow::Hidden,
            2 => Overflow::Scroll,
            _ => Overflow::Visible,
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PositionType {
    Relative,
    Absolute,
}

impl Into<stretch2::style::PositionType> for PositionType {
    fn into(self) -> stretch2::style::PositionType {
        match self {
            PositionType::Relative => stretch2::style::PositionType::Relative,
            PositionType::Absolute => stretch2::style::PositionType::Absolute,
        }
    }
}

impl From<i32> for PositionType {
    fn from(n: i32) -> Self {
        match n {
            0 => PositionType::Relative,
            1 => PositionType::Absolute,
            _ => PositionType::Relative,
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Into<stretch2::style::FlexWrap> for FlexWrap {
    fn into(self) -> stretch2::style::FlexWrap {
        match self {
            FlexWrap::NoWrap => stretch2::style::FlexWrap::NoWrap,
            FlexWrap::Wrap => stretch2::style::FlexWrap::Wrap,
            FlexWrap::WrapReverse => stretch2::style::FlexWrap::WrapReverse,
        }
    }
}

impl From<i32> for FlexWrap {
    fn from(n: i32) -> Self {
        match n {
            0 => FlexWrap::NoWrap,
            1 => FlexWrap::Wrap,
            2 => FlexWrap::WrapReverse,
            _ => FlexWrap::NoWrap,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Layout {
    #[wasm_bindgen(readonly)]
    pub width: f32,

    #[wasm_bindgen(readonly)]
    pub height: f32,

    #[wasm_bindgen(readonly)]
    pub x: f32,

    #[wasm_bindgen(readonly)]
    pub y: f32,

    #[wasm_bindgen(readonly)]
    pub childCount: usize,

    children: Vec<Layout>,
}

#[wasm_bindgen]
impl Layout {
    fn new(allocator: &Allocator, node: stretch2::node::Node) -> Layout {
        let stretch = allocator.stretch.borrow();
        let layout = stretch.layout(node).unwrap();
        let children = stretch.children(node).unwrap();

        Layout {
            width: layout.size.width,
            height: layout.size.height,
            x: layout.location.x,
            y: layout.location.y,
            childCount: children.len(),
            children: children.into_iter().map(|child| Layout::new(allocator, child)).collect(),
        }
    }

    #[wasm_bindgen]
    pub fn child(&self, at: usize) -> Layout {
        self.children[at].clone()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Allocator {
    stretch: Rc<RefCell<stretch2::node::Stretch>>,
}

#[wasm_bindgen]
impl Allocator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { stretch: Rc::new(RefCell::new(stretch2::node::Stretch::new())) }
    }
}

#[wasm_bindgen]
pub struct Node {
    allocator: Allocator,
    node: stretch2::node::Node,
    style: JsValue,

    #[wasm_bindgen(readonly)]
    pub childCount: usize,
}

#[wasm_bindgen]
impl Node {
    #[wasm_bindgen(constructor)]
    pub fn new(allocator: &Allocator, style: &JsValue) -> Self {
        Self {
            allocator: allocator.clone(),
            node: allocator.stretch.borrow_mut().new_node(parse_style(&style), &[]).unwrap(),
            style: style.clone(),
            childCount: 0,
        }
    }

    #[wasm_bindgen(js_name = setMeasure)]
    pub fn set_measure(&mut self, measure: &JsValue) {
        let measure = Function::from(measure.clone());

        self.allocator
            .stretch
            .borrow_mut()
            .set_measure(
                self.node,
                Some(stretch2::node::MeasureFunc::Boxed(Box::new(move |constraints| {
                    use stretch2::number::OrElse;

                    let widthConstraint = if let stretch2::number::Number::Defined(val) = constraints.width {
                        val.into()
                    } else {
                        JsValue::UNDEFINED
                    };

                    let heightConstaint = if let stretch2::number::Number::Defined(val) = constraints.height {
                        val.into()
                    } else {
                        JsValue::UNDEFINED
                    };

                    if let Ok(result) = measure.call2(&JsValue::UNDEFINED, &widthConstraint, &heightConstaint) {
                        let width = get_f32(&result, "width");
                        let height = get_f32(&result, "height");

                        if width.is_some() && height.is_some() {
                            return stretch2::geometry::Size { width: width.unwrap(), height: height.unwrap() };
                        }
                    }

                    constraints.map(|v| v.or_else(0.0))
                }))),
            )
            .unwrap();
    }

    #[wasm_bindgen(js_name = addChild)]
    pub fn add_child(&mut self, child: &Node) {
        self.allocator.stretch.borrow_mut().add_child(self.node, child.node).unwrap();
        self.childCount += 1;
    }

    #[wasm_bindgen(js_name = removeChild)]
    pub fn remove_child(&mut self, child: &Node) {
        self.allocator.stretch.borrow_mut().remove_child(self.node, child.node).unwrap();
        self.childCount -= 1;
    }

    #[wasm_bindgen(js_name = replaceChildAtIndex)]
    pub fn replace_child_at_index(&mut self, index: usize, child: &Node) {
        self.allocator.stretch.borrow_mut().replace_child_at_index(self.node, index, child.node).unwrap();
    }

    #[wasm_bindgen(js_name = removeChildAtIndex)]
    pub fn remove_child_at_index(&mut self, index: usize) {
        self.allocator.stretch.borrow_mut().remove_child_at_index(self.node, index).unwrap();
        self.childCount -= 1;
    }

    #[wasm_bindgen(js_name = getStyle)]
    pub fn get_style(&self) -> JsValue {
        self.style.clone()
    }

    #[wasm_bindgen(js_name = setStyle)]
    pub fn set_style(&mut self, style: &JsValue) {
        self.allocator.stretch.borrow_mut().set_style(self.node, parse_style(style)).unwrap();
        self.style = style.clone();
    }

    #[wasm_bindgen(js_name = markDirty)]
    pub fn mark_dirty(&mut self) {
        self.allocator.stretch.borrow_mut().mark_dirty(self.node).unwrap()
    }

    #[wasm_bindgen(js_name = isDirty)]
    pub fn is_dirty(&self) -> bool {
        self.allocator.stretch.borrow().dirty(self.node).unwrap()
    }

    #[wasm_bindgen(js_name = computeLayout)]
    pub fn compute_layout(&mut self, size: &JsValue) -> Layout {
        self.allocator
            .stretch
            .borrow_mut()
            .compute_layout(
                self.node,
                stretch2::geometry::Size {
                    width: if let Some(val) = get_f32(size, "width") {
                        stretch2::number::Number::Defined(val)
                    } else {
                        stretch2::number::Number::Undefined
                    },
                    height: if let Some(val) = get_f32(size, "height") {
                        stretch2::number::Number::Defined(val)
                    } else {
                        stretch2::number::Number::Undefined
                    },
                },
            )
            .unwrap();
        Layout::new(&self.allocator, self.node)
    }
}

fn parse_style(style: &JsValue) -> stretch2::style::Style {
    stretch2::style::Style {
        display: get_i32(style, "display").map(|i| Display::from(i).into()).unwrap_or_default(),
        position_type: get_i32(style, "positionType").map(|i| PositionType::from(i).into()).unwrap_or_default(),
        direction: get_i32(style, "direction").map(|i| Direction::from(i).into()).unwrap_or_default(),
        flex_direction: get_i32(style, "flexDirection").map(|i| FlexDirection::from(i).into()).unwrap_or_default(),
        flex_wrap: get_i32(style, "flexWrap").map(|i| FlexWrap::from(i).into()).unwrap_or_default(),
        overflow: get_i32(style, "overflow").map(|i| Overflow::from(i).into()).unwrap_or_default(),
        align_items: get_i32(style, "alignItems").map(|i| AlignItems::from(i).into()).unwrap_or_default(),
        align_self: get_i32(style, "alignSelf").map(|i| AlignSelf::from(i).into()).unwrap_or_default(),
        align_content: get_i32(style, "alignContent").map(|i| AlignContent::from(i).into()).unwrap_or_default(),
        justify_content: get_i32(style, "justifyContent").map(|i| JustifyContent::from(i).into()).unwrap_or_default(),

        position: stretch2::geometry::Rect {
            start: get_dimension(style, "start"),
            end: get_dimension(style, "end"),
            top: get_dimension(style, "top"),
            bottom: get_dimension(style, "bottom"),
        },

        margin: stretch2::geometry::Rect {
            start: get_dimension(style, "marginStart"),
            end: get_dimension(style, "marginEnd"),
            top: get_dimension(style, "marginTop"),
            bottom: get_dimension(style, "marginBottom"),
        },

        padding: stretch2::geometry::Rect {
            start: get_dimension(style, "paddingStart"),
            end: get_dimension(style, "paddingEnd"),
            top: get_dimension(style, "paddingTop"),
            bottom: get_dimension(style, "paddingBottom"),
        },

        border: stretch2::geometry::Rect {
            start: get_dimension(style, "borderStart"),
            end: get_dimension(style, "borderEnd"),
            top: get_dimension(style, "borderTop"),
            bottom: get_dimension(style, "borderBottom"),
        },

        flex_grow: get_f32(style, "flexGrow").unwrap_or(0.0),
        flex_shrink: get_f32(style, "flexShrink").unwrap_or(1.0),
        flex_basis: get_dimension(style, "flexBasis"),

        size: stretch2::geometry::Size {
            width: get_size_dimension(style, "width"),
            height: get_size_dimension(style, "height"),
        },

        min_size: stretch2::geometry::Size {
            width: get_size_dimension(style, "minWidth"),
            height: get_size_dimension(style, "minHeight"),
        },

        max_size: stretch2::geometry::Size {
            width: get_size_dimension(style, "maxWidth"),
            height: get_size_dimension(style, "maxHeight"),
        },

        aspect_ratio: get_f32(style, "aspectRatio")
            .map(stretch2::number::Number::Defined)
            .unwrap_or(stretch2::number::Number::Undefined),
    }
}

fn get_size_dimension(obj: &JsValue, key: &str) -> stretch2::style::Dimension {
    let dimension = get_dimension(obj, key);
    match dimension {
        stretch2::style::Dimension::Undefined => stretch2::style::Dimension::Auto,
        _ => dimension,
    }
}

fn get_dimension(obj: &JsValue, key: &str) -> stretch2::style::Dimension {
    if has_key(obj, key) {
        if let Ok(val) = Reflect::get(obj, &key.into()) {
            if let Some(number) = val.as_f64() {
                return stretch2::style::Dimension::Points(number as f32);
            }
            if let Some(string) = val.as_string() {
                if string == "auto" {
                    return stretch2::style::Dimension::Auto;
                }
                if let Ok(number) = string.parse::<f32>() {
                    return stretch2::style::Dimension::Points(number);
                }
                if string.ends_with('%') {
                    let len = string.len();
                    if let Ok(number) = string[..len - 1].parse::<f32>() {
                        return stretch2::style::Dimension::Percent(number / 100.0);
                    }
                }
            }
        }
    }
    stretch2::style::Dimension::Undefined
}

fn get_i32(obj: &JsValue, key: &str) -> Option<i32> {
    if has_key(obj, key) {
        if let Ok(val) = Reflect::get(obj, &key.into()) {
            return val.as_f64().map(|v| v as i32);
        }
    }
    None
}

fn get_f32(obj: &JsValue, key: &str) -> Option<f32> {
    if has_key(obj, key) {
        if let Ok(val) = Reflect::get(obj, &key.into()) {
            return val.as_f64().map(|v| v as f32);
        }
    }
    None
}

fn has_key(obj: &JsValue, key: &str) -> bool {
    if let Ok(exists) = Reflect::has(obj, &key.into()) {
        exists
    } else {
        false
    }
}
