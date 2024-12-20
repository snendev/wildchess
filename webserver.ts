import { serveDir, serveFile } from "jsr:@std/http/file-server";

Deno.serve((request: Request) => {
  const path = new URL(request.url).pathname;

  if (
    path.startsWith("/assets")
       || path.endsWith(".html")
       || path.endsWith(".js")
       || path.endsWith('.wasm')
  ) {
    return serveDir(request, {
      fsRoot: "dist",
      urlRoot: "",
      showIndex: true,
    });
  }

  return serveFile(request, "dist/index.html");
});
