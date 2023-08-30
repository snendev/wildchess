use egui_extras::RetainedImage;

use chess_gameplay::chess::{
    pieces::{Behavior, Pattern, TargetMode},
    team::Team,
};

pub(crate) fn wild_behavior_icon(
    behavior: &Behavior,
    team: Team,
    is_king: bool,
) -> (RetainedImage, String) {
    let svg_source = build_svg(behavior.clone(), team, is_king);
    (
        RetainedImage::from_svg_str(
            format!("svg-{:?}-{:?}", team, behavior),
            svg_source.as_str(),
        )
        // issues building the svg are developer-error, panic so that we can catch these errors
        .unwrap(),
        svg_source,
    )
}

// svg generation utilities

fn build_svg(behavior: Behavior, team: Team, is_king: bool) -> String {
    format!(
        r#"<svg
    width="1000"
    height="1000"
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
        behavior_nodes(behavior, team),
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

// larger piece svg
// fn build_piece_paths(team: Team) -> String {
//     let fill = match team {
//         Team::White => "#ffffff",
//         Team::Black => "#000000",
//     };
//     format!(
//         r#"<path
//             style="fill:{};stroke:#000000;stroke-width:20px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
//             d="m 399,605 h 200 c 4,0 6,2 6,6 v 14 H 395 v -14 c 0,-4 0,-6 4,-6 z"
//         />
//         <path
//             style="fill:{};stroke:#000000;stroke-width:20px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
//             d="m 395,590 h 210 c 0,-10 0,-16 -5,-20 -25,-10 -55,-18 -60,-39 0,-1 0,-1 1.0069,-1 H 555 c 5,0 5,-2 5,-5 0,-3 0,-5 -5,-5 h -12 c -3,0 -4,-1 -6,-3 -7,-7 -28.03327,-46 -17,-61 1,-1 1,-1 2,-1 h 8 c 5,0 5,-4 5,-7 0,-3 0.0185,-7 -5,-7 -20,0 -20,-11 -25,-26 25,-40 -40,-40 -15,0 -5,15 -5,26 -25,26 -5,0 -5,4 -5,7 0,3 -0.0185,7 5,7 h 7.98815 C 474,455 474,455 475,456 c 10.9375,15 -5,54 -12,61 -2,2 -3,3 -6,3 h -12 c -5,0 -5,2 -5,5 0,3 0,5 5,5 l 14.01886,-1.2e-4 C 460,529.99988 460,530.00809 460,531 c -5,20.99988 -35,29 -60,39 -5,4 -5,10 -5,20 z"
//         />"#,
//         fill, fill,
//     )
// }

fn build_piece_paths(team: Team) -> String {
    let (fill,) = match team {
        Team::White => ("#ffffff",),
        Team::Black => ("#000000",),
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
}

fn circle(position: NodePosition, color_hex: &str) -> String {
    format!(
        r#"<circle
        style="fill:{}"
        cx="{}"
        cy="{}"
        r="40"
    />"#,
        color_hex, position.anchor_x, position.anchor_y,
    )
}

fn horizontal_arrow(position: NodePosition, color_hex: &str) -> String {
    format!(
        r#"<path
            style="fill:none;stroke:{};stroke-width:30px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
            d="M {},{} h {}130"
        />
        <path
            style="fill:none;stroke:{};stroke-width:30px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
            d="m {},465 l {}30,35 {}30,35"
        />"#,
        color_hex,
        position.anchor_x,
        position.anchor_y,
        if position.dx.is_positive() { "" } else { "-" },
        color_hex,
        position.anchor_x + position.dx.signum() * 100,
        if position.dx.is_positive() { "" } else { "-" },
        if position.dx.is_positive() { "-" } else { "" },
    )
}

fn vertical_arrow(position: NodePosition, color_hex: &str) -> String {
    format!(
        r#"<path
            style="fill:none;stroke:{};stroke-width:30px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
            d="M {},{} v {}130"
        />
        <path
            style="fill:none;stroke:{};stroke-width:30px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
            d="m 465,{} l 35,{}30 35,{}30"
        />"#,
        color_hex,
        position.anchor_x,
        position.anchor_y,
        if position.dy.is_positive() { "" } else { "-" },
        color_hex,
        position.anchor_y + position.dy.signum() * 100,
        if position.dy.is_positive() { "" } else { "-" },
        if position.dy.is_positive() { "-" } else { "" },
    )
}

fn diagonal_arrow(position: NodePosition, color_hex: &str) -> String {
    let target_x = position.anchor_x + position.dx.signum() * 100;
    let target_y = position.anchor_y + position.dy.signum() * 100;
    format!(
        r#"<path
            style="fill:none;stroke:{};stroke-width:30px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
            d="m {},{} L {},{}"
        />
        <path
            style="fill:none;stroke:{};stroke-width:30px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
            d="m {},{} v {}40 h {}40"
        />"#,
        color_hex,
        position.anchor_x,
        position.anchor_y,
        target_x,
        target_y,
        color_hex,
        target_x,
        target_y + position.dy.signum() * -40,
        if position.dy.is_positive() { "" } else { "-" },
        if position.dx.is_positive() { "-" } else { "" },
    )
}

fn arrow(position: NodePosition, color_hex: &str) -> String {
    match (position.dx, position.dy) {
        (0, 0) => panic!("Invalid step ({},{}) for arrow", position.dx, position.dy),
        (_, 0) => horizontal_arrow(position, color_hex),
        (0, _) => vertical_arrow(position, color_hex),
        _ => diagonal_arrow(position, color_hex),
    }
}

fn calculate_node_positions(
    step_x: u8,
    step_y: i16,
    radius: u8,
    team: Team,
) -> impl Iterator<Item = NodePosition> {
    let x: i32 = step_x.into();
    let y: i32 = step_y.into();
    let radius: i32 = radius.into();
    let y = y * match team {
        Team::White => -1,
        Team::Black => 1,
    };

    let y = -(y * radius + y.signum());
    if x == 0 {
        vec![(0, y)].into_iter()
    } else {
        let x = x * radius + 1;
        vec![(x, y), (-x, y)].into_iter()
    }
    .map(|(dx, dy)| {
        let cy = 500 - dy * 100;
        let cx = 500 + dx * 100;
        NodePosition::new(cx, cy, dx, -dy)
    })
}

enum Symbol {
    Circle,
    Arrow,
}

fn pattern_nodes(pattern: Pattern, team: Team) -> String {
    let color_hex = match pattern.target_mode {
        TargetMode::Attacking => "#000000",
        TargetMode::Moving => "#0000ff",
        TargetMode::OnlyAttacking => "#ff0000",
    };

    let anchors: Vec<(NodePosition, Symbol)> = match pattern.range {
        None => calculate_node_positions(pattern.step.x, pattern.step.y, 2, team)
            .map(|node| (node, Symbol::Arrow))
            .collect(),
        Some(range) => (1..=range.min(3))
            .flat_map(|radius| {
                calculate_node_positions(pattern.step.x, pattern.step.y, radius, team)
            })
            .map(|node| (node, Symbol::Circle))
            .collect(),
    };

    anchors
        .into_iter()
        .map(|(position, symbol)| match symbol {
            Symbol::Arrow => arrow(position, color_hex),
            Symbol::Circle => circle(position, color_hex),
        })
        .collect::<Vec<_>>()
        .join("\n   ")
}

// builds a set of symbols to decorate the piece tile with patterns that describe its behavior options
fn behavior_nodes(behavior: Behavior, team: Team) -> String {
    behavior
        .patterns
        .into_iter()
        .map(|pattern| pattern_nodes(pattern, team))
        .collect::<Vec<_>>()
        .join("\n")
}
