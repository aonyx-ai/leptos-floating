use leptos::web_sys::{HtmlButtonElement, HtmlDivElement, HtmlElement};
use leptos_floating::*;
use wasm_bindgen_test::__rt::wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Positions a floating div against a real button in the DOM
///
/// The button sits at left 100, top 200 with width 120 and height 40. The expected position is
/// x = button.left = 100 and y = button.top + button.height + side_offset = 200 + 40 + 4 = 244.
/// Both elements join the DOM so that `getBoundingClientRect` works.
#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn test_use_floating_with_real_dom_elements() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();

    let button = document
        .create_element("button")
        .unwrap()
        .dyn_into::<HtmlButtonElement>()
        .unwrap();

    button.set_inner_text("Test Button");

    let button_element: &HtmlElement = button.as_ref();
    let button_style = button_element.style();
    button_style.set_property("position", "absolute").unwrap();
    button_style.set_property("left", "100px").unwrap();
    button_style.set_property("top", "200px").unwrap();
    button_style.set_property("width", "120px").unwrap();
    button_style.set_property("height", "40px").unwrap();

    let div = document
        .create_element("div")
        .unwrap()
        .dyn_into::<HtmlDivElement>()
        .unwrap();

    div.set_inner_text("Floating Content");
    let div_element: &HtmlElement = div.as_ref();
    let div_style = div_element.style();
    div_style.set_property("position", "absolute").unwrap();

    body.append_child(&button).unwrap();
    body.append_child(&div).unwrap();

    let options = FloatingOptions::builder()
        .side(Side::Bottom)
        .align(Align::Start)
        .side_offset(4.0)
        .align_offset(0.0)
        .build();

    let position = calculate_position(&button, &div, options).unwrap();

    assert_eq!(
        position.x(),
        100.0,
        "Floating should be positioned at x=100"
    );
    assert_eq!(
        position.y(),
        244.0,
        "Floating should be positioned at y=244"
    );
    assert_eq!(position.side(), Side::Bottom);
    assert_eq!(position.align(), Align::Start);

    body.remove_child(&button).unwrap();
    body.remove_child(&div).unwrap();
}

/// Tests the positioning function with known values in the browser
///
/// The floating width 80 and height 30 do not matter for `Align::Start`. The position is
/// x = reference_x and y = reference_y + height + side_offset.
#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn test_calculate_position_from_rect_in_browser() {
    let options = FloatingOptions::builder()
        .side(Side::Bottom)
        .align(Align::Start)
        .side_offset(4.0)
        .align_offset(0.0)
        .build();

    let position = calculate_position_from_rect()
        .reference_x(100.0)
        .reference_y(200.0)
        .reference_width(120.0)
        .reference_height(40.0)
        .floating_width(80.0)
        .floating_height(30.0)
        .options(options)
        .call()
        .unwrap();

    assert_eq!(position.x(), 100.0);
    assert_eq!(position.y(), 244.0);
    assert_eq!(position.side(), Side::Bottom);
    assert_eq!(position.align(), Align::Start);
}
