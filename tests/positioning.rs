use leptos_floating::*;

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

/// The reference width is 120 and the floating width is 80, so the center offset is half of 120 minus 80, which is 20
#[test]
fn align_center_centers_on_bottom_side_x() {
    let options = FloatingOptions::builder()
        .side(Side::Bottom)
        .align(Align::Center)
        .align_offset(0.0)
        .build();

    let pos = position_for(options);

    assert_eq!(
        pos.x(),
        120.0,
        "Center align should center floating on reference (100 + 20)"
    );
}

/// The reference width is 120 and the floating width is 80, so the center offset is half of 120 minus 80, which is 20
#[test]
fn align_center_centers_on_bottom_side_y() {
    let options = FloatingOptions::builder()
        .side(Side::Bottom)
        .align(Align::Center)
        .align_offset(0.0)
        .build();

    let pos = position_for(options);

    assert_eq!(pos.y(), 240.0);
}

#[test]
fn align_center_centers_on_left_side_x() {
    let options = FloatingOptions::builder()
        .side(Side::Left)
        .align(Align::Center)
        .align_offset(0.0)
        .build();

    let pos = position_for(options);

    assert_eq!(pos.x(), 100.0);
}

#[test]
fn align_center_centers_on_left_side_y() {
    let options = FloatingOptions::builder()
        .side(Side::Left)
        .align(Align::Center)
        .align_offset(0.0)
        .build();

    let pos = position_for(options);

    assert_eq!(
        pos.y(),
        205.0,
        "Center align should center floating on reference (200 + (40-30)/2)"
    );
}

/// The reference height is 40 and the floating height is 30, so the center offset is half of 40 minus 30, which is 5
#[test]
fn align_center_centers_on_right_side_x() {
    let options = FloatingOptions::builder()
        .side(Side::Right)
        .align(Align::Center)
        .align_offset(0.0)
        .build();

    let pos = position_for(options);

    assert_eq!(pos.x(), 220.0);
}

/// The reference height is 40 and the floating height is 30, so the center offset is half of 40 minus 30, which is 5
#[test]
fn align_center_centers_on_right_side_y() {
    let options = FloatingOptions::builder()
        .side(Side::Right)
        .align(Align::Center)
        .align_offset(0.0)
        .build();

    let pos = position_for(options);

    assert_eq!(
        pos.y(),
        205.0,
        "Center align should center floating on reference (200 + (40-30)/2)"
    );
}

#[test]
fn align_center_centers_on_top_side_x() {
    let options = FloatingOptions::builder()
        .side(Side::Top)
        .align(Align::Center)
        .align_offset(0.0)
        .build();

    let pos = position_for(options);

    assert_eq!(
        pos.x(),
        120.0,
        "Center align should center floating on reference (100 + (120-80)/2)"
    );
}

#[test]
fn align_center_centers_on_top_side_y() {
    let options = FloatingOptions::builder()
        .side(Side::Top)
        .align(Align::Center)
        .align_offset(0.0)
        .build();

    let pos = position_for(options);

    assert_eq!(pos.y(), 200.0);
}

/// The center shift is 100 plus half of 120 minus 80, which is 120, and the align offset 5 makes 125
#[test]
fn align_center_with_align_offset_on_bottom_x() {
    let options = FloatingOptions::builder()
        .side(Side::Bottom)
        .align(Align::Center)
        .align_offset(5.0)
        .build();

    let pos = position_for(options);

    assert_eq!(pos.x(), 125.0);
}

/// The center shift is 100 plus half of 120 minus 80, which is 120, and the align offset 5 makes 125
#[test]
fn align_center_with_align_offset_on_bottom_y() {
    let options = FloatingOptions::builder()
        .side(Side::Bottom)
        .align(Align::Center)
        .align_offset(5.0)
        .build();

    let pos = position_for(options);

    assert_eq!(pos.y(), 240.0);
}

/// The reference width is 120 and the floating width is 80, so the end offset is 120 minus 80, which is 40
#[test]
fn align_end_aligns_on_bottom_side_x() {
    let options = FloatingOptions::builder()
        .side(Side::Bottom)
        .align(Align::End)
        .align_offset(0.0)
        .build();

    let pos = position_for(options);

    assert_eq!(
        pos.x(),
        140.0,
        "End align should align floating to end of reference (100 + 120 - 80)"
    );
}

/// The reference width is 120 and the floating width is 80, so the end offset is 120 minus 80, which is 40
#[test]
fn align_end_aligns_on_bottom_side_y() {
    let options = FloatingOptions::builder()
        .side(Side::Bottom)
        .align(Align::End)
        .align_offset(0.0)
        .build();

    let pos = position_for(options);

    assert_eq!(pos.y(), 240.0);
}

#[test]
fn align_end_aligns_on_left_side_x() {
    let options = FloatingOptions::builder()
        .side(Side::Left)
        .align(Align::End)
        .align_offset(0.0)
        .build();

    let pos = position_for(options);

    assert_eq!(pos.x(), 100.0);
}

#[test]
fn align_end_aligns_on_left_side_y() {
    let options = FloatingOptions::builder()
        .side(Side::Left)
        .align(Align::End)
        .align_offset(0.0)
        .build();

    let pos = position_for(options);

    assert_eq!(
        pos.y(),
        210.0,
        "End align should align floating to end of reference (200 + 40 - 30)"
    );
}

/// The reference height is 40 and the floating height is 30, so the end offset is 40 minus 30, which is 10
#[test]
fn align_end_aligns_on_right_side_x() {
    let options = FloatingOptions::builder()
        .side(Side::Right)
        .align(Align::End)
        .align_offset(0.0)
        .build();

    let pos = position_for(options);

    assert_eq!(pos.x(), 220.0);
}

/// The reference height is 40 and the floating height is 30, so the end offset is 40 minus 30, which is 10
#[test]
fn align_end_aligns_on_right_side_y() {
    let options = FloatingOptions::builder()
        .side(Side::Right)
        .align(Align::End)
        .align_offset(0.0)
        .build();

    let pos = position_for(options);

    assert_eq!(
        pos.y(),
        210.0,
        "End align should align floating to end of reference (200 + 40 - 30)"
    );
}

#[test]
fn align_end_aligns_on_top_side_x() {
    let options = FloatingOptions::builder()
        .side(Side::Top)
        .align(Align::End)
        .align_offset(0.0)
        .build();

    let pos = position_for(options);

    assert_eq!(
        pos.x(),
        140.0,
        "End align should align floating to end of reference (100 + 120 - 80)"
    );
}

#[test]
fn align_end_aligns_on_top_side_y() {
    let options = FloatingOptions::builder()
        .side(Side::Top)
        .align(Align::End)
        .align_offset(0.0)
        .build();

    let pos = position_for(options);

    assert_eq!(pos.y(), 200.0);
}

/// The end shift is 200 plus 40 minus 30, which is 210, and the align offset 5 makes 215
#[test]
fn align_end_with_align_offset_on_right_x() {
    let options = FloatingOptions::builder()
        .side(Side::Right)
        .align(Align::End)
        .align_offset(5.0)
        .build();

    let pos = position_for(options);

    assert_eq!(pos.x(), 220.0);
}

/// The end shift is 200 plus 40 minus 30, which is 210, and the align offset 5 makes 215
#[test]
fn align_end_with_align_offset_on_right_y() {
    let options = FloatingOptions::builder()
        .side(Side::Right)
        .align(Align::End)
        .align_offset(5.0)
        .build();

    let pos = position_for(options);

    assert_eq!(pos.y(), 215.0);
}

#[test]
fn align_offset_shifts_x_for_bottom_side_x() {
    let options = FloatingOptions::builder()
        .side(Side::Bottom)
        .align(Align::Start)
        .align_offset(10.0)
        .build();

    let pos = position_for(options);

    assert_eq!(
        pos.x(),
        110.0,
        "align_offset should shift x for Bottom side"
    );
}

#[test]
fn align_offset_shifts_x_for_bottom_side_y() {
    let options = FloatingOptions::builder()
        .side(Side::Bottom)
        .align(Align::Start)
        .align_offset(10.0)
        .build();

    let pos = position_for(options);

    assert_eq!(
        pos.y(),
        240.0,
        "align_offset should NOT shift y for Bottom side"
    );
}

/// For Top and Bottom sides the cross-axis is X, so the align offset shifts X
#[test]
fn align_offset_shifts_x_for_top_side_x() {
    let options = FloatingOptions::builder()
        .side(Side::Top)
        .align(Align::Start)
        .align_offset(10.0)
        .build();

    let pos = position_for(options);

    assert_eq!(pos.x(), 110.0, "align_offset should shift x for Top side");
}

/// For Top and Bottom sides the cross-axis is X, so the align offset shifts X
#[test]
fn align_offset_shifts_x_for_top_side_y() {
    let options = FloatingOptions::builder()
        .side(Side::Top)
        .align(Align::Start)
        .align_offset(10.0)
        .build();

    let pos = position_for(options);

    assert_eq!(
        pos.y(),
        200.0,
        "align_offset should NOT shift y for Top side"
    );
}

/// For Left and Right sides the cross-axis is Y, so the align offset shifts Y
#[test]
fn align_offset_shifts_y_for_left_side_x() {
    let options = FloatingOptions::builder()
        .side(Side::Left)
        .align(Align::Start)
        .align_offset(10.0)
        .build();

    let pos = position_for(options);

    assert_eq!(
        pos.x(),
        100.0,
        "align_offset should NOT shift x for Left side"
    );
}

/// For Left and Right sides the cross-axis is Y, so the align offset shifts Y
#[test]
fn align_offset_shifts_y_for_left_side_y() {
    let options = FloatingOptions::builder()
        .side(Side::Left)
        .align(Align::Start)
        .align_offset(10.0)
        .build();

    let pos = position_for(options);

    assert_eq!(pos.y(), 210.0, "align_offset should shift y for Left side");
}

#[test]
fn align_offset_shifts_y_for_right_side_x() {
    let options = FloatingOptions::builder()
        .side(Side::Right)
        .align(Align::Start)
        .align_offset(10.0)
        .build();

    let pos = position_for(options);

    assert_eq!(
        pos.x(),
        220.0,
        "align_offset should NOT shift x for Right side"
    );
}

#[test]
fn align_offset_shifts_y_for_right_side_y() {
    let options = FloatingOptions::builder()
        .side(Side::Right)
        .align(Align::Start)
        .align_offset(10.0)
        .build();

    let pos = position_for(options);

    assert_eq!(pos.y(), 210.0, "align_offset should shift y for Right side");
}
