importScripts(
  "/wasm/chess_app_web.js",
);

// define worker event handlers
self.onmessage = onMessage;

let app;

function onMessage(event) {
  console.log(event);
  if (app !== undefined && event.data.kind === "start") {
    app.start_game();
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

runApp();
