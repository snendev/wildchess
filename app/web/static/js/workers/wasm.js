importScripts(
  "/wasm/chess_app_web.js",
);

// define worker event handlers
self.onmessage = onMessage;

let app;
runApp();

let connected = false;
let inGame = false;
let myTeam = "white";

function onMessage(event) {
  //   console.log(event);
  if (app === undefined) {
    console.warn("message received before app is instantiated; ignoring");
    return;
  }
  switch (event.data.kind) {
    case "start-game": {
      app.start_game();
      return;
    }
    case "remove-board": {
      app.remove_board();
      app.update();
      postMessage({
        kind: "position",
        position: null,
        lastMove: null,
      });
      // reset state
      connected = false;
      inGame = false;
      myTeam = "white";
      currentPosition = null;
      lastMove = null;
      currentIcons = null;
      return;
    }
    case "play-move": {
      // TODO enable premove
      app.update();
      const targets = app.get_target_squares(event.data.source)?.map((square) =>
        square.get_representation()
      );
      const pieceTeam = app.get_piece_team(event.data.source);
      if (
        pieceTeam && pieceTeam === myTeam &&
        targets?.includes(event.data.target)
      ) {
        // set state to wait on a result from server
        // send move event
        app.trigger_move(event.data.source, event.data.target);
      } else {
        // reset board
        postMessage({
          kind: "position",
          position: currentPosition,
          lastMove,
        });
      }
      return;
    }
    case "request-targets": {
      postMessage({
        kind: "targets",
        source: event.data.source,
        targets: app.get_target_squares(event.data.source)?.map((square) =>
          square.get_representation()
        ),
      });
      return;
    }
    default: {
      throw new Error("Unexpected message received: {}", event.data);
    }
  }
}

let last_player_count = 0;

// [piece, square][]
let currentPosition = null;
let lastMove = null;
let currentIcons = null;

async function runApp() {
  // initialize the wasm
  await wasm_bindgen("/wasm/chess_app_web_bg.wasm");

  // build the bevy app
  app = new wasm_bindgen.WasmApp();

  // loop update calls
  while (true) {
    app.update();

    // check connections
    if (!connected && app.is_connected()) {
      connected = true;
      console.log("connected to server!");
    }

    // check game status
    if (!inGame && app.is_in_game()) {
      inGame = true;
      console.log("connected to a game!");
      app.update();
      const orientation = app.get_my_team();
      myTeam = orientation;
      console.log(orientation);
      postMessage({ kind: "orientation", orientation: orientation ?? "white" });
    }

    // track player counts
    const player_count = app.get_player_count();
    if (player_count !== last_player_count) {
      last_player_count = player_count;
      postMessage({
        kind: "player-count",
        count: app.get_player_count(),
      });
    }

    // track piece positions
    const position = Object.fromEntries(
      app.get_piece_positions().map((
        position,
      ) => [
        position.square().get_representation(),
        position.piece().get_representation(),
      ]),
    );
    if (!deepEqual(position, currentPosition)) {
      currentPosition = position;
      const newLastMove = app.get_last_move()?.map((square) =>
        square.get_representation()
      );
      lastMove = newLastMove;
      postMessage({
        kind: "position",
        position,
        lastMove: newLastMove,
      });
    }

    // track piece icons
    const icons = Object.fromEntries(
      app.get_icons().map((icon) => {
        const piece = icon.get_piece();
        return [piece, sanitizeIconSource(icon.to_source())];
      }),
    );
    if (!deepEqual(icons, currentIcons)) {
      currentIcons = icons;
      postMessage({ kind: "piece-icons", icons });
    }

    // request next update
    await new Promise((resolve) => {
      setTimeout(resolve, 50);
    });
  }
}

function sanitizeIconSource(source) {
  return source.replaceAll("\\n", " ")
    .replaceAll("\n", " ")
    .replaceAll('\\"', '"')
    .trim();
}

function deepEqual(obj1, obj2) {
  if (obj1 === obj2) {
    return true;
  }
  if (isPrimitive(obj1) || isPrimitive(obj2)) {
    return obj1 === obj2;
  }
  if (Object.keys(obj1).length !== Object.keys(obj2).length) {
    return false;
  }
  for (const key in obj1) {
    if (!(key in obj2)) return false;
    if (!deepEqual(obj1[key], obj2[key])) return false;
  }
  return true;
}

function isPrimitive(obj) {
  return obj !== Object(obj);
}
