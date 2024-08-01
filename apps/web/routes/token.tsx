import { Handlers } from "$fresh/server.ts";

const SERVER_ORIGIN = addHttps(Deno.env.get("SERVER_ORIGIN")) ?? "http://localhost";
const SERVER_TOKEN_PORT = Deno.env.get("SERVER_TOKEN_PORT") ?? "7637";

function addHttps(origin?: string): string | null {
    if (!origin) return null;
    if (origin.startsWith("https://") || origin.startsWith("http://")) {
        return origin;
    } else {
        return `https://${origin}`;
    }
}

export const handler: Handlers<string> = {
    async GET(_req, _ctx) {
        const response = await fetch(`${SERVER_ORIGIN}:${SERVER_TOKEN_PORT}`);
        return new Response(await response.text());
    }
}
