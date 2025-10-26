// Deno edge runner that calls the same Node runner via subprocess for parity
const p = new Deno.Command("node", { args: ["runtime/runner.js"] });
const { code, stdout, stderr } = await p.output();
await Deno.stdout.write(stdout);
await Deno.stderr.write(stderr);
Deno.exit(code);
