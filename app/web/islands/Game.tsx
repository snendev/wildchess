import { useState, useMemo, useCallback, useRef, useEffect, VNode } from "preact/hooks";

import Board from "./Board.tsx";

interface WasmGameProps {
  name: string
  description: VNode,
}

export default function WasmGame({ name, description }: WasmGameProps) {
  const { tick, getPosition, getIcons, lastMoveSquares, getTargets, movePiece } = useWasmGame(name);

  return null
  //   return (
  //     <Board
  //       getIcons={getIcons}
  //       lastMoveSquares={lastMoveSquares}
  //       getPosition={getPosition}
  //       getTargets={getTargets}
  //       movePiece={movePiece}
  //       tick={tick}
  //     />
  //   );
}

function useWasmGame(game_name: string) {
  const [app, setApp] = useState(null);
  const [tick, setTick] = useState(null);
  const [lastMoveSquares, setLastMoveSquares] = useState<[string, string] | null>(null);

  useEffect(() => {
    async function initWasm() {
      const { WasmApp } = await import(`/wasm/${game_name}.js`);
      const app = new WasmApp();
      app.run();
      // app.start_game();
      // // need to update twice so that icons exist
      // app.update();
      // app.update();
      // setApp(app);
    }
    initWasm();
  }, []);

  const start = useMemo(() => +Date.now() / 1000, []);
  const value = useRef(0);
  useInterval(useCallback(() => {
    if (!app) return null;
    app.send_server_message(value.current++);
    app.update();
  }, [app, start]), 20);

  const movePiece = useCallback((pieceSquare: string, targetSquare: string) => {
    if (app === null) return;
    const didMove = app.trigger_move(pieceSquare, targetSquare);
    if (didMove) {
      app.update();
      setLastMoveSquares([pieceSquare, targetSquare]);
    }
    return didMove;
  }, [app]);

  const getTargetSquares = useCallback(
    (square: string): string[] | null =>
      app?.get_target_squares(square)?.map((square) => square.get_representation()) ?? null,
    [app],
  );

  const getPosition = useCallback(() => {
    if (!app) return null
    return Object.fromEntries(
      app.get_piece_positions()
        .map((position) => [position.square().get_representation(), position.piece().get_representation()])
    )
  }, [app]);

  // todo: enable promotion
  const getIcons = useCallback(() => {
    if (!app) return null;
    return Object.fromEntries(
      app.get_icons().map((icon) => {
        const piece = icon.get_piece();
        return [piece, sanitizeIconSource(icon.to_source())];
      })
    );
  }, [app])

  return { tick, getPosition, getIcons, movePiece, getTargets: getTargetSquares, lastMoveSquares };
}

function sanitizeIconSource(source: string): string {
  const trimmedSource = source.replaceAll('\\n', ' ')
    .replaceAll('\n', ' ')
    .replaceAll("\\\"", "\"")
    .replaceAll("\"", "'")
    .trim();
  const parser = new DOMParser();
  const svg = parser.parseFromString(trimmedSource, 'image/svg+xml');
  const serializer = new XMLSerializer();
  return serializer.serializeToString(svg);
}

function useInterval(callback, delay) {
  const savedCallback = useRef();

  useEffect(() => {
    savedCallback.current = callback;
  });

  useEffect(() => {
    function tick() {
      savedCallback.current();
    }

    let id = setInterval(tick, delay);
    return () => clearInterval(id);
  }, [delay]);
}
