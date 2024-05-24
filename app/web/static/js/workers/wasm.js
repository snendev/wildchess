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
  if (app === undefined) {
    console.warn("message received before app is instantiated; ignoring");
    return;
  }
  switch (event.data.kind) {
    case "request-game": {
      const gameRequest = makeGameRequest(event.data.variant, event.data.clock);
      app.request_game(gameRequest);
      postMessage({ kind: "network-state", state: "awaiting-game" });
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
    case "leave-game": {
      app.leave_game();
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
    case "play-move": {
      // TODO enable premove
      app.update();

      // perform a pre-check in order to render the board optimistically
      if (
        // it's my turn
        app.is_my_turn() &&
        // the selected piece is one of my pieces
        app.get_piece_team(event.data.source) === myTeam &&
        // the target square is one of the allowed targets
        app.get_target_squares(event.data.source)?.map((square) =>
          square.get_representation()
        )?.includes(event.data.target)
      ) {
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
    case "select-promotion": {
      // confirm the context is correct
      if (
        promotionOptions !== null &&
        app.is_my_turn()
      ) {
        app.select_promotion(promotionOptions, event.data.promotionIndex);
        promotionOptions = null;
      }
      return;
    }
    default: {
      throw new Error("Unexpected message received: " + event.data);
    }
  }
}

let last_player_count = 0;

// [piece, square][]
let currentPosition = null;
let lastMove = null;
let currentIcons = null;
let promotionOptions = null;

async function runApp() {
  const [response] = await Promise.all([
    // send a network request to get the server token
    fetch("/token"),
    // and while waiting initialize the wasm
    wasm_bindgen("/wasm/chess_app_web_bg.wasm"),
  ]);

  const token = await response.text();

  // build the bevy app
  app = new wasm_bindgen.WasmApp(token);

  // loop update calls
  while (true) {
    app.update();

    // check connections
    if (!connected && app.is_connected()) {
      connected = true;
      postMessage({ kind: "network-state", state: "connected" });
    }

    // check game status
    if (!inGame && app.is_in_game()) {
      inGame = true;
      postMessage({ kind: "network-state", state: "in-game" });
      app.update();
      const orientation = app.get_my_team();
      myTeam = orientation;
      postMessage({ kind: "orientation", orientation: orientation ?? "white" });
    }

    if (inGame && !app.is_in_game()) {
      inGame = false;
      console.log("exited game");
      postMessage({ kind: "network-state", state: "connected" });
      app.update();
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

    const maybePromotions = app.get_promotion_request();
    if (maybePromotions != null) {
      postMessage({
        kind: "require-promotion",
        icons: maybePromotions.icons().map((icon) => sanitizeIconSource(icon)),
      });
      promotionOptions = maybePromotions;
    }

    const winningTeam = app.is_game_over();
    if (winningTeam != null) {
      postMessage({
        kind: "gameover",
        winningTeam: winningTeam.get_team(),
      });
    }

    // request next update
    await new Promise((resolve) => {
      setTimeout(resolve, 50);
    });
  }
}

function makeGameRequest(variant, clock) {
  let gameRequest = wasm_bindgen.WasmGameRequest.new();
  switch (variant) {
    case "featured-1": {
      gameRequest = gameRequest.with_featured_game_one();
      break;
    }
    case "featured-2": {
      gameRequest = gameRequest.with_featured_game_two();
      break;
    }
    case "featured-3": {
      gameRequest = gameRequest.with_featured_game_three();
      break;
    }
    case "wild": {
      gameRequest = gameRequest.with_wild_game();
      break;
    }
    case null:
      break;
    default:
      throw new Error("Unexpected game request kind: " + variant);
  }
  switch (clock) {
    case "classical": {
      gameRequest = gameRequest.with_classical_clock();
      break;
    }
    case "rapid": {
      gameRequest = gameRequest.with_rapid_clock();
      break;
    }
    case "blitz": {
      gameRequest = gameRequest.with_blitz_clock();
      break;
    }
    case "bullet": {
      gameRequest = gameRequest.with_bullet_clock();
      break;
    }
    case null: {
      break;
    }
    default:
      throw new Error("Unexpected game clock: " + clock);
  }
  return gameRequest;
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
