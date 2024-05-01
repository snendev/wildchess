import * as wasm from 'chess_app_web2';

init();

function init() {
  async function _init() {
    const app = new wasm.WasmApp();
    app.start_game();
    console.log('App created!');
    while (true) {
      console.log('update');
      app.update();
      await new Promise((resolve) => setTimeout(resolve, 50));
    }
    console.log('App disconnected!');
  }

  _init();
}
