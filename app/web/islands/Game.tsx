import { useState, useMemo, useCallback, useEffect, VNode } from "preact/hooks";

import Board from "./Board.tsx";

function useWasmGame(game_name: string) {
    const [app, setApp] = useState(null);
    const [iconMap, setIconMap] = useState({});
    const [lastMoveSquares, setLastMoveSquares] = useState<[string, string] | null>(null);

    useEffect(() => {
        async function initWasm() {
            const { WasmApp } = await import(`/wasm/${game_name}.js`);
            const app = new WasmApp();
            setApp(app);
        }
        initWasm();
    }, []);

    useEffect(() => {
        if (app === null) return;
        app.start_game();
        app.update();
        app.update();
        const map = Object.fromEntries(
            app.get_icons().map((icon) => {
                const key = icon.key();
                return [key, sanitizeIconSource(icon.to_source())];
            })
        );
        setIconMap(map);
    }, [app]);

    const movePiece = useCallback((pieceSquare: string, targetSquare: string) => {
        if (app === null) return;
        const didMove = app.trigger_move(pieceSquare, targetSquare);
        if (didMove) {
            app.update();
            setLastMoveSquares([pieceSquare, targetSquare]);
        }
        console.log(app.check_game_state());
        return didMove;
    }, [app]);

    const getTargetSquares = useCallback(
        (square: string): string[] | null =>
            app?.get_target_squares(square)?.map((square) => square.to_string()) ?? null,
        [app],
    );

    return { lastMoveSquares, iconMap, movePiece, getTargetSquares };
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

interface WasmGameProps {
    name: string
    description: VNode,
}

export default function WasmGame({ name, description }: WasmGameProps) {
    const [selectedSquare, setSelectedSquare] = useState(null);
    const { iconMap, lastMoveSquares, getTargetSquares, movePiece } = useWasmGame(name);
    const targetedSquares = useMemo(
        () => selectedSquare ? getTargetSquares(selectedSquare) : null,
        [getTargetSquares, selectedSquare],
    );
    return (
        <Board
            iconMap={iconMap}
            selectedSquare={selectedSquare}
            selectSquare={setSelectedSquare}
            lastMoveSquares={lastMoveSquares}
            targetedSquares={targetedSquares}
            movePiece={movePiece}
        />
    );
}
