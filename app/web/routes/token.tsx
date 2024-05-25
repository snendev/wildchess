import { Handlers } from "$fresh/server.ts";

const SERVER_ORIGIN = Deno.env.get("SERVER_ORIGIN") ?? "127.0.0.1";
const SERVER_TOKEN_PORT = Deno.env.get("SERVER_TOKEN_PORT") ?? "7637";

export const handler: Handlers<string> = {
    async GET(_req, _ctx) {
        const response = await fetch(`http://${SERVER_ORIGIN}:${SERVER_TOKEN_PORT}`);
        return new Response(await response.text());
    }
}
