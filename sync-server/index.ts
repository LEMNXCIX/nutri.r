import { serve } from "bun";
import { existsSync, writeFileSync, readFileSync } from "fs";

const VAULT_FILE = "vault.json";

// Inicializar vault.json si no existe
if (!existsSync(VAULT_FILE)) {
    writeFileSync(VAULT_FILE, JSON.stringify({ last_updated: "1970-01-01T00:00:00Z" }, null, 2));
}

serve({
    port: 3000,
    async fetch(req) {
        const url = new URL(req.url);

        // Endpoint para obtener el vault actual
        if (req.method === "GET" && url.pathname === "/vault") {
            try {
                const data = readFileSync(VAULT_FILE, "utf-8");
                return new Response(data, {
                    headers: { "Content-Type": "application/json" },
                });
            } catch (err) {
                return new Response("Error leyendo el vault", { status: 500 });
            }
        }

        // Endpoint para actualizar el vault (LWW)
        if (req.method === "POST" && url.pathname === "/vault") {
            try {
                const remoteData = await req.json();
                const localData = JSON.parse(readFileSync(VAULT_FILE, "utf-8"));

                const remoteTS = new Date(remoteData.last_updated || 0).getTime();
                const localTS = new Date(localData.last_updated || 0).getTime();

                if (remoteTS > localTS) {
                    writeFileSync(VAULT_FILE, JSON.stringify(remoteData, null, 2));
                    return new Response(JSON.stringify({ status: "updated", last_updated: remoteData.last_updated }), {
                        headers: { "Content-Type": "application/json" },
                    });
                } else {
                    return new Response(JSON.stringify({ status: "ignored", reason: "stale_data", current_ts: localData.last_updated }), {
                        status: 409, // Conflict (stale data)
                        headers: { "Content-Type": "application/json" },
                    });
                }
            } catch (err) {
                return new Response(JSON.stringify({ error: "Invalid JSON or server error" }), { status: 400 });
            }
        }

        return new Response("Nutri-R Sync Server is running", { status: 200 });
    },
});

console.log("🚀 Nutri-R Sync Server running on http://localhost:3000");
