use games::chess::{
    pattern::{CaptureMode, CapturePattern, Pattern},
    pieces::Orientation,
    team::Team,
};

// svg generation utilities

pub(crate) fn wild_behavior_icon(
    patterns: &[Pattern],
    team: Team,
    orientation: Orientation,
    is_king: bool,
) -> String {
    format!(
        r#"<svg
    width="100%"
    height="100%"
    viewBox="0 0 1000 1000"
    version="1.1"
    xmlns="http://www.w3.org/2000/svg"
    xmlns:svg="http://www.w3.org/2000/svg"
>
    <g>
        {}
        {}
    </g>
</svg>"#,
        piece_nodes(team, is_king),
        behavior_nodes(
            patterns,
            match team {
                Team::White => orientation,
                Team::Black => orientation.flip(),
            }
        ),
    )
}

fn piece_nodes(team: Team, is_king: bool) -> String {
    if is_king {
        build_king_paths(team)
    } else {
        build_piece_paths(team)
    }
}

fn build_king_paths(team: Team) -> String {
    let (fill,) = match team {
        Team::White => ("#ffffff",),
        Team::Black => ("#000000",),
    };
    format!(
        r#"<path
            style="fill:{};stroke:#000000;stroke-width:20px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
            d="m 399,605 h 200 c 4,0 6,2 6,6 v 14 H 395 v -14 c 0,-4 0,-6 4,-6 z"
        />
        <path
            style="fill:{};stroke:#000000;stroke-width:20px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
            d="m 395,590 h 210 l 10,-124 c 21,-3 -5,-23 -1,-1 l -59,50 -54,-115 c 19,-18 -21,-18 -2,0 l -49,115 -64,-50 c 4,-22 -22,-2 -1,1 z"
        />"#,
        fill, fill
    )
}

fn build_piece_paths(team: Team) -> String {
    let fill = match team {
        Team::White => "#ffffff",
        Team::Black => "#000000",
    };
    format!(
        r#"<path
            style="fill:{};stroke:#000000;stroke-width:20px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
            d="m 395,624 h 210 c 0,-10 0,-34 -10,-39 -25,-10 -50,-14 -55,-35 0,-1 0,-1 1.0069,-1 H 565 c 10,0 10,-5 10,-15 0,-10 0,-15 -10,-15 h -25 c -3,0 -4,-1 -6,-3 l -19,-52 c 40,-70 -75,-70 -35,0 l -14,52 c -2,2 -3,3 -6,3 h -25 c -10,0 -10,5 -10,15 0,10 0,15 10,15 l 24.01886,-1.2e-4 C 460,548.99988 460,549.00809 460,550 c -5,20.99988 -30,25 -55,35 -10,5 -10,29 -10,39 z"
        />"#,
        fill,
    )
}

struct NodePosition {
    anchor_x: i32,
    anchor_y: i32,
    dx: i32,
    dy: i32,
}

impl NodePosition {
    fn new(anchor_x: i32, anchor_y: i32, dx: i32, dy: i32) -> Self {
        NodePosition {
            anchor_x,
            anchor_y,
            dx,
            dy,
        }
    }

    pub fn calculate(step_x: i16, step_y: i16, radius: usize, orientation: Orientation) -> Self {
        let x: i32 = step_x.into();
        let y: i32 = step_y.into();
        let radius: i32 = radius as i32;
        let y: i32 = y * match orientation {
            Orientation::Up => -1,
            Orientation::Down => 1,
            _ => unimplemented!("Pieces on left and right are currently unimplemented"),
        };

        let dy = -(y * radius + y.signum());
        let dx = x * radius + x.signum();
        let cy = 500 - dy * 100;
        let cx = 500 + dx * 100;
        NodePosition::new(cx, cy, dx, -dy)
    }
}

fn circle(position: &NodePosition, color_hex: &str) -> String {
    format!(
        r#"<circle
        style="fill:{}"
        cx="{}"
        cy="{}"
        r="30"
    />"#,
        color_hex, position.anchor_x, position.anchor_y,
    )
}

fn circle_range_text(position: &NodePosition, range: usize) -> String {
    let center_x = position.anchor_x - 40;
    let center_y = position.anchor_y + 40;
    format!(
        r#"<text fill="black" x={center_x} y={center_y} font-size="128" font-weight="bold">{range}</text>"#
    )
}

fn horizontal_arrow(position: &NodePosition, color_hex: &str, dash: bool) -> String {
    let arrow_length = if dash { 180 } else { 100 };
    let anchor_x = position.anchor_x;
    let anchor_y = position.anchor_y;
    let arrow_offset_signum = if position.dx.is_positive() { "" } else { "-" };
    let arrowhead_x = anchor_x + position.dx.signum() * arrow_length;
    let arrowhead_signum_1 = if position.dx.is_positive() { "" } else { "-" };
    let arrowhead_signum_2 = if position.dx.is_positive() { "-" } else { "" };
    let dash_style = if dash {
        r#"stroke-dasharray=40,40"#
    } else {
        ""
    };
    format!(
        r#"<path
            style="fill:none;stroke:{color_hex};stroke-width:30px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
            d="M {anchor_x},{anchor_y} h {arrow_offset_signum}130"
            {dash_style}
        />
        <path
            style="fill:none;stroke:{color_hex};stroke-width:30px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
            d="m {arrowhead_x},465 l {arrowhead_signum_1}30,35 {arrowhead_signum_2}30,35"
        />"#,
    )
}

fn vertical_arrow(position: &NodePosition, color_hex: &str, dash: bool) -> String {
    let arrow_length = if dash { 180 } else { 100 };
    let anchor_x = position.anchor_x;
    let anchor_y = position.anchor_y;
    let arrow_offset_signum = if position.dy.is_positive() { "" } else { "-" };
    let arrowhead_y = position.anchor_y + position.dy.signum() * arrow_length;
    let arrowhead_signum_1 = if position.dy.is_positive() { "" } else { "-" };
    let arrowhead_signum_2 = if position.dy.is_positive() { "-" } else { "" };
    let dash_style = if dash {
        r#"stroke-dasharray=40,40"#
    } else {
        ""
    };
    format!(
        r#"<path
            style="fill:none;stroke:{color_hex};stroke-width:30px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
            d="M {anchor_x},{anchor_y} v {arrow_offset_signum}130"
            {dash_style}
        />
        <path
            style="fill:none;stroke:{color_hex};stroke-width:30px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
            d="m 465,{arrowhead_y} l 35,{arrowhead_signum_1}30 35,{arrowhead_signum_2}30"
        />"#,
    )
}

fn diagonal_arrow(position: &NodePosition, color_hex: &str, dash: bool) -> String {
    let arrow_length = if dash { 180 } else { 100 };
    let target_x = position.anchor_x + position.dx.signum() * arrow_length;
    let target_y = position.anchor_y + position.dy.signum() * arrow_length;
    let anchor_x = position.anchor_x;
    let anchor_y = position.anchor_y;
    let arrowhead_y = target_y + position.dy.signum() * -40;
    let arrowhead_signum_x = if position.dy.is_positive() { "" } else { "-" };
    let arrowhead_signum_y = if position.dx.is_positive() { "-" } else { "" };
    let dash_style = if dash {
        r#"stroke-dasharray=40,40"#
    } else {
        ""
    };
    format!(
        r#"<path
            style="ill:{color_hex};stroke:{color_hex};stroke-width:30px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
            d="m {anchor_x},{anchor_y} L {target_x},{target_y}"
            {dash_style}
        />
        <path
            style="ill:{color_hex};stroke:{color_hex};stroke-width:30px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
            d="m {target_x},{arrowhead_y} v {arrowhead_signum_x}40 h {arrowhead_signum_y}40"
        />"#,
    )
}

fn arrow(position: &NodePosition, color_hex: &str, dash: bool) -> String {
    match (position.dx, position.dy) {
        (0, 0) => panic!("Invalid step ({},{}) for arrow", position.dx, position.dy),
        (_, 0) => horizontal_arrow(position, color_hex, dash),
        (0, _) => vertical_arrow(position, color_hex, dash),
        _ => diagonal_arrow(position, color_hex, dash),
    }
}

fn cross(position: &NodePosition, color_hex: &str) -> String {
    let x11 = position.anchor_x as f32 + 35.;
    let y11 = position.anchor_y as f32 + 35.;
    let x12 = position.anchor_x as f32 + -35.;
    let y12 = position.anchor_y as f32 + -35.;
    let x21 = position.anchor_x as f32 + -35.;
    let y21 = position.anchor_y as f32 + 35.;
    let x22 = position.anchor_x as f32 + 35.;
    let y22 = position.anchor_y as f32 + -35.;

    format!(
        r#"<line
        style="stroke:{color_hex};stroke-width:30px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
        x1="{x11}"
        y1="{y11}"
        x2="{x12}"
        y2="{y12}"
    />
    <line
        style="stroke:{color_hex};stroke-width:30px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
        x1="{x21}"
        y1="{y21}"
        x2="{x22}"
        y2="{y22}"
    />"#,
    )
}

fn pattern_nodes(pattern: &Pattern, orientation: Orientation) -> String {
    let color_hex = match pattern.capture.map(|capture| capture.mode) {
        None => "#0000ff",
        Some(CaptureMode::CanCapture) => "#000000",
        Some(CaptureMode::MustCapture) => "#ff0000",
    };

    let movements = pattern.scanner.step.movements();
    let is_en_passant = pattern
        .capture
        .is_some_and(|capture| matches!(capture.pattern, CapturePattern::CaptureInPassing));
    match pattern.scanner.range {
        None => movements
            .into_iter()
            .map(|(x, y)| NodePosition::calculate(x, y, 1, orientation))
            .map(|node| {
                let shape = arrow(&node, color_hex, is_en_passant);
                if is_en_passant {
                    format!("{}{}", shape, cross(&node, color_hex))
                } else {
                    shape
                }
            })
            .collect::<Vec<_>>(),
        Some(range) => movements
            .iter()
            .flat_map(move |(x, y)| {
                let nodes = (1..=range.min(2))
                    .map(|radius| NodePosition::calculate(*x, *y, radius, orientation))
                    .collect::<Vec<_>>();
                let mut elements = nodes
                    .iter()
                    .map(|node| {
                        if is_en_passant {
                            cross(node, color_hex)
                        } else {
                            circle(node, color_hex)
                        }
                    })
                    .collect::<Vec<_>>();
                if range == 3 {
                    let node: NodePosition = NodePosition::calculate(*x, *y, 3, orientation);
                    if is_en_passant {
                        elements.push(cross(&node, color_hex));
                    } else {
                        elements.push(circle(&node, color_hex));
                    }
                }
                if range > 3 {
                    let node: NodePosition = NodePosition::calculate(*x, *y, 3, orientation);
                    elements.push(circle_range_text(&node, range));
                }
                elements
            })
            .collect::<Vec<_>>(),
    }
    .join("\n    ")
}

// builds a set of symbols to decorate the piece tile with patterns that describe its behavior options
fn behavior_nodes(patterns: &[Pattern], orientation: Orientation) -> String {
    patterns
        .iter()
        .map(|pattern| pattern_nodes(pattern, orientation))
        .collect::<Vec<_>>()
        .join("\n")
}
