importScripts(
  "/wasm/chess_app_web.js",
);

// define worker event handlers
self.onmessage = onMessage;

let app;
runApp();

function onMessage(event) {
  console.log(event);
  if (app === undefined) {
    console.warn("message received before app is instantiated; ignoring");
    return;
  }
  switch (event.data.kind) {
    case "setup-board": {
      app.setup_board();
      postMessage({
        kind: "piece-icons",
        icons: Object.fromEntries(
          app.get_icons().map((icon) => {
            const piece = icon.get_piece();
            return [piece, sanitizeIconSource(icon.to_source())];
          }),
        ),
      });
      return;
    }
    case "remove-board": {
      app.remove_board();
      return;
    }
    case "play-move": {
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

async function runApp() {
  // initialize the wasm
  await wasm_bindgen("/wasm/chess_app_web_bg.wasm");

  // build the bevy app
  app = new wasm_bindgen.WasmApp();

  // loop update calls
  while (true) {
    app.update();
    await new Promise((resolve) => {
      setTimeout(resolve, 50);
    });
  }
}

function sanitizeIconSource(source) {
  const trimmedSource = source.replaceAll("\\n", " ")
    .replaceAll("\n", " ")
    .replaceAll('\\"', '"')
    .replaceAll('"', "'")
    .trim();
  const parser = new DOMParser();
  const svg = parser.parseFromString(trimmedSource, "image/svg+xml");
  const serializer = new XMLSerializer();
  return serializer.serializeToString(svg);
}
