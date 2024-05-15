importScripts(
  "/wasm/chess_app_web.js",
);

// define worker event handlers
self.onmessage = onMessage;

let app;
runApp();

let connected = false;
let inGame = false;

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
      return;
    }
    case "play-move": {
      // TODO enable premove
      app.update();
      const targets = app.get_target_squares(event.data.source).map((square) =>
        square.get_representation()
      );
      console.log({ source: event.data.source, targets });
      if (targets.includes(event.data.target)) {
        console.log("executing move!");
        app.trigger_move(event.data.source, event.data.target);
        app.update();
        postMessage({
          kind: "position",
          position: Object.fromEntries(
            app.get_piece_positions().map((
              position,
            ) => [
              position.square().get_representation(),
              position.piece().get_representation(),
            ]),
          ),
          lastMove: [event.data.source, event.data.target],
        });
        return;
      }
      postMessage({
        kind: "position",
        position: Object.fromEntries(
          app.get_piece_positions().map((
            position,
          ) => [
            position.square().get_representation(),
            position.piece().get_representation(),
          ]),
        ),
        lastMove: undefined,
      });
      return;
    }
    case "request-targets": {
      postMessage({
        kind: "targets",
        source: event.data.source,
        targets: app.get_target_squares(event.data.source).map((square) =>
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

async function runApp() {
  // initialize the wasm
  await wasm_bindgen("/wasm/chess_app_web_bg.wasm");

  // build the bevy app
  app = new wasm_bindgen.WasmApp();

  // loop update calls
  while (true) {
    app.update();
    if (!connected && app.is_connected()) {
      connected = true;
      console.log("connected to server!");
    }
    if (!inGame && app.is_in_game()) {
      inGame = true;
      console.log("connected to a game!");
      app.update();
      const icons = Object.fromEntries(
        app.get_icons().map((icon) => {
          const piece = icon.get_piece();
          return [piece, sanitizeIconSource(icon.to_source())];
        }),
      );
      postMessage({
        kind: "piece-icons",
        icons: icons,
      });
    }
    const player_count = app.get_player_count();
    if (player_count !== last_player_count) {
      last_player_count = player_count;
      postMessage({
        kind: "player-count",
        count: app.get_player_count(),
      });
    }
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
