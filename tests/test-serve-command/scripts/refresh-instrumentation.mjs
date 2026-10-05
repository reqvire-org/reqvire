export function instrument() {
  const original = window.fetch;
  window.reqvireManifestTraffic = [];
  window.fetch = async (...args) => {
    const url = String(args[0]);
    const parsed = new URL(url, location.href);
    if (parsed.origin !== location.origin || !["/api/project-store", "/api/project-store/manifest", "/api/project-store/chunks"].includes(parsed.pathname)) return original(...args);
    const request = args[1] ?? {};
    const entry = { url, path: parsed.pathname, method: request.method ?? "GET" };
    const chunks = parsed.pathname === "/api/project-store/chunks";
    if (chunks) Object.assign(entry, JSON.parse(request.body));
    window.reqvireManifestTraffic.push(entry);
    if (chunks && window.reqvireHoldNextChunks) {
      window.reqvireHoldNextChunks = false;
      window.reqvireHeldChunks = true;
      await new Promise(resolve => {
        window.reqvireReleaseChunks = () => { window.reqvireHeldChunks = false; resolve(); };
      });
    }
    let response = await original(...args);
    entry.status = response.status;
    if (chunks && response.ok && window.reqvireChunkFault) {
      const payload = await response.clone().json();
      const first = Object.keys(payload.chunks)[0];
      entry.fault = window.reqvireChunkFault;
      if (entry.fault === "missing") delete payload.chunks[first];
      else payload.chunks[first] += " ";
      window.reqvireChunkFault = null;
      response = new Response(JSON.stringify(payload), { status: 200, headers: response.headers });
    }
    return response;
  };
}
