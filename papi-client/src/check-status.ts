// Quick script to check which chains are responding
import { WebSocket } from "ws";

const CHAINS = [
  { name: "Relay Chain", port: 9944 },
  { name: "Asset Hub", port: 9946 },
  { name: "Bulletin", port: 9947 },
];

async function checkChain(name: string, port: number): Promise<boolean> {
  return new Promise((resolve) => {
    const ws = new WebSocket(`ws://127.0.0.1:${port}`);
    const timeout = setTimeout(() => {
      ws.close();
      resolve(false);
    }, 3000);

    ws.on("open", () => {
      clearTimeout(timeout);
      // Send a simple RPC request
      ws.send(JSON.stringify({ jsonrpc: "2.0", id: 1, method: "system_health", params: [] }));
    });

    ws.on("message", (data) => {
      try {
        const response = JSON.parse(data.toString());
        if (response.result) {
          ws.close();
          resolve(true);
        }
      } catch {
        ws.close();
        resolve(false);
      }
    });

    ws.on("error", () => {
      clearTimeout(timeout);
      resolve(false);
    });
  });
}

async function main() {
  console.log("Checking chain status...\n");

  for (const chain of CHAINS) {
    const status = await checkChain(chain.name, chain.port);
    const icon = status ? "✓" : "✗";
    const color = status ? "\x1b[32m" : "\x1b[31m";
    console.log(`${color}${icon}\x1b[0m ${chain.name} (port ${chain.port}): ${status ? "UP" : "DOWN"}`);
  }
  console.log();
}

main();
