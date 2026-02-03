const http = require('http');
const fs = require('fs');
const path = require('path');

const VAULT_FILE = path.join(__dirname, 'vault.json');
const PORT = 3000;

const server = http.createServer((req, res) => {
    const url = new URL(req.url, `http://${req.headers.host}`);

    // CORS headers
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET, POST, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

    if (req.method === 'OPTIONS') {
        res.writeHead(204);
        res.end();
        return;
    }

    // Endpoint para obtener el vault actual
    if (url.pathname === '/vault') {
        if (req.method === 'GET') {
            if (!fs.existsSync(VAULT_FILE)) {
                res.writeHead(404, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ error: 'Vault not found' }));
                return;
            }
            try {
                const data = fs.readFileSync(VAULT_FILE, 'utf-8');
                res.writeHead(200, { 'Content-Type': 'application/json' });
                res.end(data);
            } catch (err) {
                res.writeHead(500);
                res.end('Error leyendo el vault');
            }
            return;
        }

        if (req.method === 'POST') {
            let body = '';
            req.on('data', chunk => { body += chunk.toString(); });
            req.on('end', () => {
                try {
                    const remoteData = JSON.parse(body);
                    let localTS = 0;

                    if (fs.existsSync(VAULT_FILE)) {
                        const fileContent = fs.readFileSync(VAULT_FILE, 'utf-8');
                        if (fileContent.trim()) {
                            const localData = JSON.parse(fileContent);
                            localTS = new Date(localData.last_updated || 0).getTime();
                        }
                    }

                    const remoteTS = new Date(remoteData.last_updated || 0).getTime();

                    // LWW logic: Solo guardamos si el remoto es más reciente que el local
                    // (O si el local no existe, remoteTS siempre será >= 0)
                    if (remoteTS >= localTS) {
                        fs.writeFileSync(VAULT_FILE, JSON.stringify(remoteData, null, 2));
                        res.writeHead(200, { 'Content-Type': 'application/json' });
                        res.end(JSON.stringify({ status: 'updated', last_updated: remoteData.last_updated }));
                    } else {
                        res.writeHead(409, { 'Content-Type': 'application/json' });
                        res.end(JSON.stringify({
                            status: 'ignored',
                            reason: 'stale_data',
                            current_ts: new Date(localTS).toISOString()
                        }));
                    }
                } catch (err) {
                    console.error('Server error processing request:', err);
                    res.writeHead(400, { 'Content-Type': 'application/json' });
                    res.end(JSON.stringify({ error: err.message || 'Invalid JSON or server error' }));
                }
            });
            return;
        }
    }

    res.writeHead(200, { 'Content-Type': 'text/plain' });
    res.end('Nutri-R Sync Server is running. Use /vault endpoint.');
});

server.listen(PORT, '0.0.0.0', () => {
    console.log(`🚀 Nutri-R Sync Server running on http://localhost:${PORT}`);
    console.log(`📁 Vault storage: ${VAULT_FILE}`);
});
