use itertools::Itertools;

use bevy::{
    prelude::{Commands, Res, Resource},
    utils::HashMap,
};

use egui_extras::RetainedImage;

use wildchess_game::{
    components::{Behavior, PieceKind, StartPosition, Team},
    GamePieces, PieceConfiguration,
};

#[derive(Clone)]
pub enum PieceIcon {
    Svg(std::sync::Arc<RetainedImage>),
    Character(char),
}

#[derive(Default, Clone, Resource)]
pub struct PieceIcons(
    pub  HashMap<
        (StartPosition, Team),
        // these images should never be mutably accessed,
        // so we do not wrap them in a Mutex
        PieceIcon,
    >,
);

fn build_king_paths(team: Team) -> String {
    let (fill, stroke) = match team {
        Team::White => ("#ffffff", "#000000"),
        Team::Black => ("#000000", "#ffffff"),
    };
    format!(
        r#"
    <path
       style="fill:{};stroke:{};stroke-width:8px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
       d="m 399,605 h 200 c 4,0 6,2 6,6 v 14 H 395 v -14 c 0,-4 0,-6 4,-6 z"
       id="path8727"
    />
    <path
       style="fill:{};stroke:{};stroke-width:20px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
       d="m 395,590 h 210 l 10,-124 c 21,-3 -5,-23 -1,-1 l -59,50 -54,-115 c 19,-18 -21,-18 -2,0 l -49,115 -64,-50 c 4,-22 -22,-2 -1,1 z"
       id="path11350"
    />
"#,
        fill, stroke, fill, stroke
    )
}

fn build_piece_paths(team: Team) -> String {
    let (fill, stroke, stroke_width) = match team {
        Team::White => ("#ffffff", "#000000", 20),
        Team::Black => ("#000000", "#ffffff", 14),
    };
    format!(
        r#"
    <path
        style="fill:{};stroke:{};stroke-width:8px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
        d="m 399,605 h 200 c 4,0 6,2 6,6 v 14 H 395 v -14 c 0,-4 0,-6 4,-6 z"
        id="path8727"
    />
    <path
        style="fill:{};stroke:{};stroke-width:{}px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
        d="m 395,590 h 210 c 0,-10 0,-16 -5,-20 -25,-10 -55,-18 -60,-39 0,-1 0,-1 1.0069,-1 H 555 c 5,0 5,-2 5,-5 0,-3 0,-5 -5,-5 h -12 c -3,0 -4,-1 -6,-3 -7,-7 -28.03327,-46 -17,-61 1,-1 1,-1 2,-1 h 8 c 5,0 5,-4 5,-7 0,-3 0.0185,-7 -5,-7 -20,0 -20,-11 -25,-26 25,-40 -40,-40 -15,0 -5,15 -5,26 -25,26 -5,0 -5,4 -5,7 0,3 -0.0185,7 5,7 h 7.98815 C 474,455 474,455 475,456 c 10.9375,15 -5,54 -12,61 -2,2 -3,3 -6,3 h -12 c -5,0 -5,2 -5,5 0,3 0,5 5,5 l 14.01886,-1.2e-4 C 460,529.99988 460,530.00809 460,531 c -5,20.99988 -35,29 -60,39 -5,4 -5,10 -5,20 z"
        id="path15561"
    />
"#,
        fill, stroke, fill, stroke, stroke_width
    )
}

fn build_pawn_paths(team: Team) -> String {
    let (fill, stroke) = match team {
        Team::White => ("#ffffff", "#000000"),
        Team::Black => ("#000000", "#ffffff"),
    };
    format!(
        r#"
    <path
        style="fill:{};stroke:{};stroke-width:20px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
        d="m 395,624 h 210 c 0,-10 0,-34 -10,-39 -25,-10 -50,-14 -55,-35 0,-1 0,-1 1.0069,-1 H 565 c 10,0 10,-5 10,-15 0,-10 0,-15 -10,-15 h -25 c -3,0 -4,-1 -6,-3 l -19,-52 c 40,-70 -75,-70 -35,0 l -14,52 c -2,2 -3,3 -6,3 h -25 c -10,0 -10,5 -10,15 0,10 0,15 10,15 l 24.01886,-1.2e-4 C 460,548.99988 460,549.00809 460,550 c -5,20.99988 -30,25 -55,35 -10,5 -10,29 -10,39 z"
        id="path15561"
    />
"#,
        fill, stroke
    )
}

fn piece_paths(kind: PieceKind, team: Team) -> String {
    match kind {
        PieceKind::King => build_king_paths(team),
        PieceKind::Piece => build_piece_paths(team),
        PieceKind::Pawn => build_pawn_paths(team),
    }
}

fn build_svg(kind: PieceKind, _behavior: Behavior, team: Team) -> String {
    format!(
        r#"<svg
    width="1000"
    height="1000"
    viewBox="0 0 1000 1000"
    version="1.1"
    xmlns="http://www.w3.org/2000/svg"
    xmlns:svg="http://www.w3.org/2000/svg"
>
    <g>{}</g>
</svg>"#,
        piece_paths(kind, team),
    )
}

pub fn build_icons(pieces: &GamePieces) -> PieceIcons {
    PieceIcons(
        pieces
            .0
            .iter()
            .cartesian_product([Team::White, Team::Black])
            .flat_map(
                |((PieceConfiguration { kind, behavior, .. }, start_positions), team)| {
                    let svg = RetainedImage::from_svg_str(
                        format!("svg-{:?}-{:?}", team, start_positions),
                        build_svg(*kind, behavior.clone(), team).as_str(),
                    )
                    .unwrap();
                    let svg = PieceIcon::Svg(std::sync::Arc::new(svg));
                    start_positions
                        .iter()
                        .map(move |start_position| ((start_position.clone(), team), svg.clone()))
                },
            )
            .collect::<HashMap<_, _>>(),
    )
}

pub fn initialize_icons(mut commands: Commands, game_pieces: Res<GamePieces>) {
    commands.insert_resource(build_icons(game_pieces.as_ref()));
}
