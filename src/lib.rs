use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::{Element, HtmlElement};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Align {
    Start,
    Center,
    End,
}

#[derive(Clone, Debug, bon::Builder, getset::CopyGetters)]
#[getset(get_copy = "pub")]
pub struct FloatingOptions {
    #[builder(default = Side::Bottom)]
    side: Side,
    #[builder(default = Align::Start)]
    align: Align,
    #[builder(default = 0.0)]
    side_offset: f64,
    #[builder(default = 0.0)]
    align_offset: f64,
}

impl Default for FloatingOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}

/// Calculated position for floating elements
///
/// # Examples
///
/// ```
/// use leptos_floating::{Align, FloatingPosition, Side};
///
/// let position = FloatingPosition::builder()
///     .x(100.0)
///     .y(200.0)
///     .side(Side::Bottom)
///     .align(Align::Center)
///     .build();
///
/// assert_eq!(position.x(), 100.0);
/// ```
#[derive(Clone, Debug, bon::Builder, getset::CopyGetters)]
#[getset(get_copy = "pub")]
pub struct FloatingPosition {
    x: f64,
    y: f64,
    side: Side,
    align: Align,
}

/// Returns dynamic positioning data for floating elements
///
/// # Examples
///
/// ```no_run
/// use leptos::prelude::*;
/// use leptos_floating::{FloatingOptions, use_floating};
///
/// let floating = use_floating(NodeRef::new(), NodeRef::new(), FloatingOptions::default());
///
/// let x = floating.x().get_untracked();
///
/// assert_eq!(x, 0.0);
/// ```
#[derive(getset::CopyGetters)]
#[getset(get_copy = "pub")]
pub struct UseFloatingReturn {
    x: RwSignal<f64>,
    y: RwSignal<f64>,
    side: Signal<Side>,
    align: Signal<Align>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FloatingX(f64);

impl Default for FloatingX {
    fn default() -> Self {
        Self(0.0)
    }
}

impl From<FloatingX> for f64 {
    fn from(val: FloatingX) -> Self {
        val.0
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FloatingY(f64);

impl Default for FloatingY {
    fn default() -> Self {
        Self(0.0)
    }
}

impl From<FloatingY> for f64 {
    fn from(val: FloatingY) -> Self {
        val.0
    }
}

/// Returns the reactive position of a floating element next to a reference element
///
/// The effect reads both node refs so that it runs again once they are populated.
/// The animation frame reads them untracked, to avoid reactive context warnings.
///
/// # Examples
///
/// ```no_run
/// use leptos::prelude::*;
/// use leptos_floating::{FloatingOptions, use_floating};
///
/// let reference = NodeRef::new();
/// let floating = NodeRef::new();
///
/// let position = use_floating(reference, floating, FloatingOptions::default());
///
/// assert_eq!(position.y().get_untracked(), 0.0);
/// ```
pub fn use_floating(
    reference_ref: NodeRef<leptos::html::Button>,
    floating_ref: NodeRef<leptos::html::Div>,
    options: FloatingOptions,
) -> UseFloatingReturn {
    let x = RwSignal::new(FloatingX::default().into());
    let y = RwSignal::new(FloatingY::default().into());
    let side = RwSignal::new(options.side());
    let align = RwSignal::new(options.align());

    Effect::new({
        let options = options.clone();

        move |_| {
            let _ = (reference_ref.get(), floating_ref.get());

            request_animation_frame({
                let x = x;
                let y = y;
                let side = side;
                let align = align;
                let options = options.clone();

                move || {
                    if let (Some(reference), Some(floating)) =
                        (reference_ref.get_untracked(), floating_ref.get_untracked())
                        && let Some(position) = calculate_position(&reference, &floating, options)
                    {
                        x.set(position.x());
                        y.set(position.y());
                        side.set(position.side());
                        align.set(position.align());
                    }
                }
            });
        }
    });

    UseFloatingReturn {
        x,
        y,
        side: side.into(),
        align: align.into(),
    }
}

/// Returns the optimal position for a floating element
///
/// The position follows from the bounding rectangles of the reference and floating elements.
///
/// # Examples
///
/// ```no_run
/// use leptos::web_sys::HtmlElement;
/// use leptos_floating::{FloatingOptions, calculate_position};
///
/// fn place(reference: &HtmlElement, floating: &HtmlElement) {
///     let position = calculate_position(reference, floating, FloatingOptions::default());
///
///     assert!(position.is_some());
/// }
/// ```
pub fn calculate_position<T, U>(
    reference: &T,
    floating: &U,
    options: FloatingOptions,
) -> Option<FloatingPosition>
where
    T: AsRef<leptos::web_sys::HtmlElement>,
    U: AsRef<leptos::web_sys::HtmlElement>,
{
    let reference_element: &HtmlElement = reference.as_ref();
    let ref_rect = reference_element
        .unchecked_ref::<Element>()
        .get_bounding_client_rect();

    let floating_element: &HtmlElement = floating.as_ref();
    let float_rect = floating_element
        .unchecked_ref::<Element>()
        .get_bounding_client_rect();

    calculate_position_from_rect()
        .reference_x(ref_rect.left())
        .reference_y(ref_rect.top())
        .reference_width(ref_rect.width())
        .reference_height(ref_rect.height())
        .floating_width(float_rect.width())
        .floating_height(float_rect.height())
        .options(options)
        .call()
}

/// Returns the position for bounding rectangle values, which needs no DOM
///
/// The cross-axis is horizontal for the top and bottom sides and vertical for the others.
/// The alignment shift and the align offset both apply on the cross-axis.
///
/// # Examples
///
/// ```
/// use leptos_floating::{FloatingOptions, calculate_position_from_rect};
///
/// let position = calculate_position_from_rect()
///     .reference_x(100.0)
///     .reference_y(200.0)
///     .reference_width(120.0)
///     .reference_height(40.0)
///     .floating_width(80.0)
///     .floating_height(30.0)
///     .options(FloatingOptions::default())
///     .call()
///     .unwrap();
///
/// assert_eq!(position.y(), 240.0);
/// ```
#[bon::builder]
pub fn calculate_position_from_rect(
    reference_x: f64,
    reference_y: f64,
    reference_width: f64,
    reference_height: f64,
    floating_width: f64,
    floating_height: f64,
    options: FloatingOptions,
) -> Option<FloatingPosition> {
    let FloatingOptions {
        side,
        align,
        side_offset,
        align_offset,
    } = options;

    let is_horizontal_side = match side {
        Side::Top | Side::Bottom => true,
        Side::Right | Side::Left => false,
    };

    let (base_x, base_y) = match side {
        Side::Top => (reference_x, reference_y - side_offset),
        Side::Right => (reference_x + reference_width + side_offset, reference_y),
        Side::Bottom => (reference_x, reference_y + reference_height + side_offset),
        Side::Left => (reference_x - side_offset, reference_y),
    };

    let align_shift = match align {
        Align::Start => 0.0,
        Align::Center => {
            if is_horizontal_side {
                (reference_width - floating_width) / 2.0
            } else {
                (reference_height - floating_height) / 2.0
            }
        }
        Align::End => {
            if is_horizontal_side {
                reference_width - floating_width
            } else {
                reference_height - floating_height
            }
        }
    };

    let (x, y) = if is_horizontal_side {
        (base_x + align_shift + align_offset, base_y)
    } else {
        (base_x, base_y + align_shift + align_offset)
    };

    Some(
        FloatingPosition::builder()
            .x(x)
            .y(y)
            .side(side)
            .align(align)
            .build(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_auto_traits<T: Send + Sync + Unpin>() {}

    fn position_for(options: FloatingOptions) -> FloatingPosition {
        calculate_position_from_rect()
            .reference_x(100.0)
            .reference_y(200.0)
            .reference_width(120.0)
            .reference_height(40.0)
            .floating_width(80.0)
            .floating_height(30.0)
            .options(options)
            .call()
            .unwrap()
    }

    #[test]
    fn align_debug_center() {
        assert_eq!(format!("{:?}", Align::Center), "Center");
    }

    #[test]
    fn align_debug_end() {
        assert_eq!(format!("{:?}", Align::End), "End");
    }

    #[test]
    fn align_debug_start() {
        assert_eq!(format!("{:?}", Align::Start), "Start");
    }

    #[test]
    fn align_differs_from_other_variant() {
        assert_ne!(Align::Start, Align::Center);
    }

    #[test]
    fn align_equals_itself() {
        assert_eq!(Align::Start, Align::Start);
    }

    #[test]
    fn align_exhaustive_match_names_center() {
        let test_align = Align::Center;

        let result = match test_align {
            Align::Start => "start",
            Align::Center => "center",
            Align::End => "end",
        };

        assert_eq!(result, "center");
    }

    #[test]
    fn align_is_send_sync_and_unpin() {
        assert_auto_traits::<Align>();
    }

    #[test]
    fn calculate_position_bottom_side_keeps_align() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Bottom)
                .align(Align::Start)
                .side_offset(4.0)
                .build(),
        );

        assert_eq!(position.align(), Align::Start);
    }

    #[test]
    fn calculate_position_bottom_side_keeps_side() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Bottom)
                .align(Align::Start)
                .side_offset(1.0)
                .build(),
        );

        assert_eq!(position.side(), Side::Bottom);
    }

    #[test]
    fn calculate_position_bottom_side_x() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Bottom)
                .align(Align::Start)
                .side_offset(4.0)
                .build(),
        );

        assert_eq!(position.x(), 100.0);
    }

    #[test]
    fn calculate_position_bottom_side_y() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Bottom)
                .align(Align::Start)
                .side_offset(4.0)
                .build(),
        );

        assert_eq!(position.y(), 244.0);
    }

    #[test]
    fn calculate_position_large_offset_x() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Right)
                .align(Align::Start)
                .side_offset(1000.0)
                .build(),
        );

        assert_eq!(position.x(), 1220.0);
    }

    #[test]
    fn calculate_position_large_offset_y() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Right)
                .align(Align::Start)
                .side_offset(1000.0)
                .build(),
        );

        assert_eq!(position.y(), 200.0);
    }

    #[test]
    fn calculate_position_left_side_keeps_side() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Left)
                .align(Align::Start)
                .side_offset(1.0)
                .build(),
        );

        assert_eq!(position.side(), Side::Left);
    }

    #[test]
    fn calculate_position_left_side_x() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Left)
                .align(Align::Start)
                .side_offset(6.0)
                .build(),
        );

        assert_eq!(position.x(), 94.0);
    }

    #[test]
    fn calculate_position_left_side_y() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Left)
                .align(Align::Start)
                .side_offset(6.0)
                .build(),
        );

        assert_eq!(position.y(), 200.0);
    }

    #[test]
    fn calculate_position_negative_offset_x() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Top)
                .align(Align::Start)
                .side_offset(-10.0)
                .build(),
        );

        assert_eq!(position.x(), 100.0);
    }

    #[test]
    fn calculate_position_negative_offset_y() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Top)
                .align(Align::Start)
                .side_offset(-10.0)
                .build(),
        );

        assert_eq!(position.y(), 210.0);
    }

    #[test]
    fn calculate_position_right_side_keeps_side() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Right)
                .align(Align::Start)
                .side_offset(1.0)
                .build(),
        );

        assert_eq!(position.side(), Side::Right);
    }

    #[test]
    fn calculate_position_right_side_x() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Right)
                .align(Align::Start)
                .side_offset(12.0)
                .build(),
        );

        assert_eq!(position.x(), 232.0);
    }

    #[test]
    fn calculate_position_right_side_y() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Right)
                .align(Align::Start)
                .side_offset(12.0)
                .build(),
        );

        assert_eq!(position.y(), 200.0);
    }

    #[test]
    fn calculate_position_top_side_keeps_side() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Top)
                .align(Align::Start)
                .side_offset(1.0)
                .build(),
        );

        assert_eq!(position.side(), Side::Top);
    }

    #[test]
    fn calculate_position_top_side_x() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Top)
                .align(Align::Start)
                .side_offset(8.0)
                .build(),
        );

        assert_eq!(position.x(), 100.0);
    }

    #[test]
    fn calculate_position_top_side_y() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Top)
                .align(Align::Start)
                .side_offset(8.0)
                .build(),
        );

        assert_eq!(position.y(), 192.0);
    }

    #[test]
    fn calculate_position_zero_offset_x() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Bottom)
                .align(Align::Start)
                .side_offset(0.0)
                .build(),
        );

        assert_eq!(position.x(), 100.0);
    }

    #[test]
    fn calculate_position_zero_offset_y() {
        let position = position_for(
            FloatingOptions::builder()
                .side(Side::Bottom)
                .align(Align::Start)
                .side_offset(0.0)
                .build(),
        );

        assert_eq!(position.y(), 240.0);
    }

    #[test]
    fn floating_options_builder_align() {
        let options = FloatingOptions::builder()
            .side(Side::Top)
            .align(Align::End)
            .side_offset(16.0)
            .build();

        assert_eq!(options.align, Align::End);
    }

    #[test]
    fn floating_options_builder_align_offset() {
        let options = FloatingOptions::builder()
            .side(Side::Top)
            .align(Align::End)
            .side_offset(16.0)
            .build();

        assert_eq!(options.align_offset, 0.0);
    }

    #[test]
    fn floating_options_builder_side() {
        let options = FloatingOptions::builder()
            .side(Side::Top)
            .align(Align::End)
            .side_offset(16.0)
            .build();

        assert_eq!(options.side, Side::Top);
    }

    #[test]
    fn floating_options_builder_side_offset() {
        let options = FloatingOptions::builder()
            .side(Side::Top)
            .align(Align::End)
            .side_offset(16.0)
            .build();

        assert_eq!(options.side_offset, 16.0);
    }

    #[test]
    fn floating_options_clone_keeps_align() {
        let original = FloatingOptions::builder()
            .side(Side::Right)
            .align(Align::End)
            .side_offset(10.0)
            .align_offset(5.0)
            .build();

        let cloned = original.clone();

        assert_eq!(cloned.align, original.align);
    }

    #[test]
    fn floating_options_clone_keeps_align_offset() {
        let original = FloatingOptions::builder()
            .side(Side::Right)
            .align(Align::End)
            .side_offset(10.0)
            .align_offset(5.0)
            .build();

        let cloned = original.clone();

        assert_eq!(cloned.align_offset, original.align_offset);
    }

    #[test]
    fn floating_options_clone_keeps_side() {
        let original = FloatingOptions::builder()
            .side(Side::Right)
            .align(Align::End)
            .side_offset(10.0)
            .align_offset(5.0)
            .build();

        let cloned = original.clone();

        assert_eq!(cloned.side, original.side);
    }

    #[test]
    fn floating_options_clone_keeps_side_offset() {
        let original = FloatingOptions::builder()
            .side(Side::Right)
            .align(Align::End)
            .side_offset(10.0)
            .align_offset(5.0)
            .build();

        let cloned = original.clone();

        assert_eq!(cloned.side_offset, original.side_offset);
    }

    #[test]
    fn floating_options_custom_align() {
        let options = FloatingOptions::builder()
            .side(Side::Top)
            .align(Align::Center)
            .side_offset(8.0)
            .align_offset(4.0)
            .build();

        assert_eq!(options.align, Align::Center);
    }

    #[test]
    fn floating_options_custom_align_offset() {
        let options = FloatingOptions::builder()
            .side(Side::Top)
            .align(Align::Center)
            .side_offset(8.0)
            .align_offset(4.0)
            .build();

        assert_eq!(options.align_offset, 4.0);
    }

    #[test]
    fn floating_options_custom_side() {
        let options = FloatingOptions::builder()
            .side(Side::Top)
            .align(Align::Center)
            .side_offset(8.0)
            .align_offset(4.0)
            .build();

        assert_eq!(options.side, Side::Top);
    }

    #[test]
    fn floating_options_custom_side_offset() {
        let options = FloatingOptions::builder()
            .side(Side::Top)
            .align(Align::Center)
            .side_offset(8.0)
            .align_offset(4.0)
            .build();

        assert_eq!(options.side_offset, 8.0);
    }

    #[test]
    fn floating_options_default_align_is_default() {
        let options = FloatingOptions::default();

        assert_eq!(options.align, Align::Start);
    }

    #[test]
    fn floating_options_default_align_offset_is_default() {
        let options = FloatingOptions::default();

        assert_eq!(options.align_offset, 0.0);
    }

    #[test]
    fn floating_options_default_side_is_default() {
        let options = FloatingOptions::default();

        assert_eq!(options.side, Side::Bottom);
    }

    #[test]
    fn floating_options_default_side_offset_is_default() {
        let options = FloatingOptions::default();

        assert_eq!(options.side_offset, 0.0);
    }

    #[test]
    fn floating_options_is_send_sync_and_unpin() {
        assert_auto_traits::<FloatingOptions>();
    }

    #[test]
    fn floating_options_negative_align_offset() {
        let options = FloatingOptions::builder()
            .side_offset(-5.0)
            .align_offset(-10.0)
            .build();

        assert_eq!(options.align_offset, -10.0);
    }

    #[test]
    fn floating_options_negative_side_offset() {
        let options = FloatingOptions::builder()
            .side_offset(-5.0)
            .align_offset(-10.0)
            .build();

        assert_eq!(options.side_offset, -5.0);
    }

    #[test]
    fn floating_options_zero_align_offset() {
        let options = FloatingOptions::builder()
            .side(Side::Top)
            .side_offset(0.0)
            .align_offset(0.0)
            .build();

        assert_eq!(options.align_offset, 0.0);
    }

    #[test]
    fn floating_options_zero_side_offset() {
        let options = FloatingOptions::builder()
            .side(Side::Top)
            .side_offset(0.0)
            .align_offset(0.0)
            .build();

        assert_eq!(options.side_offset, 0.0);
    }

    #[test]
    fn floating_position_builder_sets_align() {
        let position = FloatingPosition::builder()
            .x(100.0)
            .y(200.0)
            .side(Side::Bottom)
            .align(Align::Center)
            .build();

        assert_eq!(position.align, Align::Center);
    }

    #[test]
    fn floating_position_builder_sets_side() {
        let position = FloatingPosition::builder()
            .x(100.0)
            .y(200.0)
            .side(Side::Bottom)
            .align(Align::Center)
            .build();

        assert_eq!(position.side, Side::Bottom);
    }

    #[test]
    fn floating_position_builder_sets_x() {
        let position = FloatingPosition::builder()
            .x(100.0)
            .y(200.0)
            .side(Side::Bottom)
            .align(Align::Center)
            .build();

        assert_eq!(position.x, 100.0);
    }

    #[test]
    fn floating_position_builder_sets_y() {
        let position = FloatingPosition::builder()
            .x(100.0)
            .y(200.0)
            .side(Side::Bottom)
            .align(Align::Center)
            .build();

        assert_eq!(position.y, 200.0);
    }

    #[test]
    fn floating_position_clone_keeps_align() {
        let original = FloatingPosition::builder()
            .x(150.0)
            .y(250.0)
            .side(Side::Right)
            .align(Align::End)
            .build();

        let cloned = original.clone();

        assert_eq!(cloned.align, original.align);
    }

    #[test]
    fn floating_position_clone_keeps_side() {
        let original = FloatingPosition::builder()
            .x(150.0)
            .y(250.0)
            .side(Side::Right)
            .align(Align::End)
            .build();

        let cloned = original.clone();

        assert_eq!(cloned.side, original.side);
    }

    #[test]
    fn floating_position_clone_keeps_x() {
        let original = FloatingPosition::builder()
            .x(150.0)
            .y(250.0)
            .side(Side::Right)
            .align(Align::End)
            .build();

        let cloned = original.clone();

        assert_eq!(cloned.x, original.x);
    }

    #[test]
    fn floating_position_clone_keeps_y() {
        let original = FloatingPosition::builder()
            .x(150.0)
            .y(250.0)
            .side(Side::Right)
            .align(Align::End)
            .build();

        let cloned = original.clone();

        assert_eq!(cloned.y, original.y);
    }

    #[test]
    fn floating_position_is_send_sync_and_unpin() {
        assert_auto_traits::<FloatingPosition>();
    }

    #[test]
    fn floating_x_default_is_zero() {
        assert_eq!(f64::from(FloatingX::default()), 0.0);
    }

    #[test]
    fn floating_x_is_send_sync_and_unpin() {
        assert_auto_traits::<FloatingX>();
    }

    #[test]
    fn floating_y_default_is_zero() {
        assert_eq!(f64::from(FloatingY::default()), 0.0);
    }

    #[test]
    fn floating_y_is_send_sync_and_unpin() {
        assert_auto_traits::<FloatingY>();
    }

    #[test]
    fn side_debug_bottom() {
        assert_eq!(format!("{:?}", Side::Bottom), "Bottom");
    }

    #[test]
    fn side_debug_left() {
        assert_eq!(format!("{:?}", Side::Left), "Left");
    }

    #[test]
    fn side_debug_right() {
        assert_eq!(format!("{:?}", Side::Right), "Right");
    }

    #[test]
    fn side_debug_top() {
        assert_eq!(format!("{:?}", Side::Top), "Top");
    }

    #[test]
    fn side_differs_from_other_variant() {
        assert_ne!(Side::Top, Side::Bottom);
    }

    #[test]
    fn side_equals_itself() {
        assert_eq!(Side::Top, Side::Top);
    }

    #[test]
    fn side_exhaustive_match_names_bottom() {
        let test_side = Side::Bottom;

        let result = match test_side {
            Side::Top => "top",
            Side::Right => "right",
            Side::Bottom => "bottom",
            Side::Left => "left",
        };

        assert_eq!(result, "bottom");
    }

    #[test]
    fn side_is_send_sync_and_unpin() {
        assert_auto_traits::<Side>();
    }

    #[test]
    fn use_floating_return_is_send_sync_and_unpin() {
        assert_auto_traits::<UseFloatingReturn>();
    }
}
