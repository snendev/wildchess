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
            app.start_game();
            // need to update twice so that icons exist
            app.update();
            app.update();
            const map = Object.fromEntries(
                app.get_icons().map((icon) => {
                    const piece = icon.get_piece();
                    return [piece, sanitizeIconSource(icon.to_source())];
                })
            );
            setIconMap(map);
            setApp(app);
        }
        initWasm();
    }, []);

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

    const position = useMemo(() => {
        if (!app) return null
        return Object.fromEntries(
            app.get_piece_positions()
                .map((position) => [position.square().get_representation(), position.piece().get_representation()])
        )
    }, [app]);

    return { position, lastMoveSquares, iconMap, movePiece, getTargetSquares };
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
    const { position, iconMap, lastMoveSquares, getTargetSquares, movePiece } = useWasmGame(name);
    const targetedSquares = useMemo(
        () => selectedSquare ? getTargetSquares(selectedSquare) : null,
        [getTargetSquares, selectedSquare],
    );
    return (
        <Board
            iconMap={iconMap}
            position={position}
            selectedSquare={selectedSquare}
            selectSquare={setSelectedSquare}
            lastMoveSquares={lastMoveSquares}
            targetedSquares={targetedSquares}
            movePiece={movePiece}
        />
    );
}
