// use html::Img;
use leptos::*;
// use leptos_use::{
//     use_draggable_with_options, UseDraggableCallbackArgs, UseDraggableOptions, UseDraggableReturn,
// };

use wildchess::games::chess::{
    pieces::{Mutation, PieceIdentity},
    team::Team,
};
use wildchess::wild_icons::PieceIconSvg;

#[component]
pub fn Piece(
    #[prop(into)] piece: Signal<(PieceIdentity, Team, Option<Mutation>)>,
    #[prop(into)] icon: Signal<PieceIconSvg>,
    #[prop(into)] square: Signal<Option<String>>,
    #[prop(into)] square_size: Signal<u16>,
    #[prop(into)] hidden: Signal<bool>,
) -> impl IntoView {
    // let element = create_node_ref::<Img>();

    // let UseDraggableReturn { style, .. } = use_draggable_with_options(
    //     element,
    //     UseDraggableOptions::default()
    //         .exact(true)
    //         .on_start(move |UseDraggableCallbackArgs { event, position }| {
    //             #[cfg(feature = "log")]
    //             wildchess_web::log(format!("Drag event start: {event:?} {position:?}"));
    //             true
    //         })
    //         .on_end(move |UseDraggableCallbackArgs { event, position }| {
    //             #[cfg(feature = "log")]
    //             wildchess_web::log(format!("Drag event end: {event:?} {position:?}"));
    //         }),
    // );

    let img_source = move || {
        let icon = icon.get();
        format!(
            "data:image/svg+xml;charset=utf-8,{}",
            icon.source
                .replace('#', "%23")
                .replace('"', "'")
                .replace('&', "&amp;")
        )
    };

    let name = move || {
        let (id, team, mutation) = piece.get();
        format!("{} {}", team.name(), id.name())
    };
    let key = move || {
        let (id, team, mutation) = piece.get();
        format!("{}{}", team.code(), id.code())
    };

    let classname = move || {
        format!(
            "piece {} {}",
            key(),
            if hidden.get() { "hidden" } else { "" }
        )
    };
    let style = move || {
        format!(
            "position:relative;width:{}px;height:{}px;",
            square_size.get(),
            square_size.get(),
            // style.get(),
        )
    };

    view! {
        <img
            src=img_source
            // node_ref=element
            id=key
            data-piece=key
            data-square=square
            alt=name
            data-piece=key
            class=classname
            style=style
        />
    }
}

// // -------------------------------------------------------------------------
// // Animations
// // -------------------------------------------------------------------------

// function animateSquareToSquare(src, dest, piece, completeFn) {
//     // get information about the source and destination squares
//     var $srcSquare = $("#" + squareElsIds[src]);
//     var srcSquarePosition = $srcSquare.offset();
//     var $destSquare = $("#" + squareElsIds[dest]);
//     var destSquarePosition = $destSquare.offset();

//     // create the animated piece and absolutely position it
//     // over the source square
//     var animatedPieceId = uuid();
//     $("body").append(
//         buildPieceHTML(piece, { hidden: true, id: animatedPieceId }),
//     );
//     var $animatedPiece = $("#" + animatedPieceId);
//     $animatedPiece.css({
//         display: "",
//         position: "absolute",
//         top: srcSquarePosition.top,
//         left: srcSquarePosition.left,
//     });

//     // remove original piece from source square
//     $srcSquare.find("." + CSS.piece).remove();

//     function onFinishAnimation1() {
//         // add the "real" piece to the destination square
//         $destSquare.append(buildPieceHTML(piece));

//         // remove the animated piece
//         $animatedPiece.remove();

//         // run complete function
//         if (isFunction(completeFn)) {
//         completeFn();
//         }
//     }

//     // animate the piece to the destination square
//     var opts = {
//         duration: config.moveSpeed,
//         complete: onFinishAnimation1,
//     };
//     $animatedPiece.animate(destSquarePosition, opts);
//     }

//     function animateSparePieceToSquare(piece, dest, completeFn) {
//     var srcOffset = $("#" + sparePiecesElsIds[piece]).offset();
//     var $destSquare = $("#" + squareElsIds[dest]);
//     var destOffset = $destSquare.offset();

//     // create the animate piece
//     var pieceId = uuid();
//     $("body").append(
//         buildPieceHTML(piece, { hidden: true, id: pieceId }),
//     );
//     var $animatedPiece = $("#" + pieceId);
//     $animatedPiece.css({
//         display: "",
//         position: "absolute",
//         left: srcOffset.left,
//         top: srcOffset.top,
//     });

//     // on complete
//     function onFinishAnimation2() {
//         // add the "real" piece to the destination square
//         $destSquare.find("." + CSS.piece).remove();
//         $destSquare.append(buildPieceHTML(piece, { square: dest }));

//         // remove the animated piece
//         $animatedPiece.remove();

//         // run complete function
//         if (isFunction(completeFn)) {
//         completeFn();
//         }
//     }

//     // animate the piece to the destination square
//     var opts = {
//         duration: config.moveSpeed,
//         complete: onFinishAnimation2,
//     };
//     $animatedPiece.animate(destOffset, opts);
//     }

//     // execute an array of animations
//     function doAnimations(animations, oldPos, newPos) {
//     if (animations.length === 0) return;

//     var numFinished = 0;
//     function onFinishAnimation3() {
//         // exit if all the animations aren't finished
//         numFinished = numFinished + 1;
//         if (numFinished !== animations.length) return;

//         drawPositionInstant();

//         // run their onMoveEnd function
//         if (isFunction(config.onMoveEnd)) {
//         config.onMoveEnd(deepCopy(oldPos), deepCopy(newPos));
//         }
//     }

//     for (var i = 0; i < animations.length; i++) {
//         var animation = animations[i];

//         // clear a piece
//         if (animation.type === "clear") {
//         $("#" + squareElsIds[animation.square] + " ." + CSS.piece)
//             .fadeOut(config.trashSpeed, onFinishAnimation3);

//         // add a piece with no spare pieces - fade the piece onto the square
//         } else if (animation.type === "add" && !config.sparePieces) {
//         $("#" + squareElsIds[animation.square])
//             .append(
//             buildPieceHTML(animation.piece, {
//                 hidden: true,
//                 square: animation.square,
//             }),
//             )
//             .find("." + CSS.piece)
//             .fadeIn(config.appearSpeed, onFinishAnimation3);

//         // add a piece with spare pieces - animate from the spares
//         } else if (animation.type === "add" && config.sparePieces) {
//         animateSparePieceToSquare(
//             animation.piece,
//             animation.square,
//             onFinishAnimation3,
//         );

//         // move a piece from squareA to squareB
//         } else if (animation.type === "move") {
//         animateSquareToSquare(
//             animation.source,
//             animation.destination,
//             animation.piece,
//             onFinishAnimation3,
//         );
//         }
//     }
//     }

//     // calculate an array of animations that need to happen in order to get
//     // from pos1 to pos2
//     function calculateAnimations(pos1, pos2) {
//     // make copies of both
//     pos1 = deepCopy(pos1);
//     pos2 = deepCopy(pos2);

//     var animations = [];
//     var squaresMovedTo = {};

//     // remove pieces that are the same in both positions
//     for (var i in pos2) {
//         if (!pos2.hasOwnProperty(i)) continue;

//         if (pos1.hasOwnProperty(i) && pos1[i] === pos2[i]) {
//         delete pos1[i];
//         delete pos2[i];
//         }
//     }

//     // find all the "move" animations
//     for (i in pos2) {
//         if (!pos2.hasOwnProperty(i)) continue;

//         var closestPiece = findClosestPiece(pos1, pos2[i], i);
//         if (closestPiece) {
//         animations.push({
//             type: "move",
//             source: closestPiece,
//             destination: i,
//             piece: pos2[i],
//         });

//         delete pos1[closestPiece];
//         delete pos2[i];
//         squaresMovedTo[i] = true;
//         }
//     }

//     // "add" animations
//     for (i in pos2) {
//         if (!pos2.hasOwnProperty(i)) continue;

//         animations.push({
//         type: "add",
//         square: i,
//         piece: pos2[i],
//         });

//         delete pos2[i];
//     }

//     // "clear" animations
//     for (i in pos1) {
//         if (!pos1.hasOwnProperty(i)) continue;

//         // do not clear a piece if it is on a square that is the result
//         // of a "move", ie: a piece capture
//         if (squaresMovedTo.hasOwnProperty(i)) continue;

//         animations.push({
//         type: "clear",
//         square: i,
//         piece: pos1[i],
//         });

//         delete pos1[i];
//     }

//     return animations;
//     }
