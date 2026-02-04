const { spawn } = require('child_process');
const os = require('os');

const isWindows = os.platform() === 'win32';
const command = isWindows ? 'npx.cmd' : 'npx';
const args = ['tailwindcss', '-i', './input.css', '-o', './styles.css'];

console.log(`Running CSS build: ${command} ${args.join(' ')}`);

const child = spawn(command, args, { stdio: 'inherit', shell: isWindows });

child.on('exit', (code) => {
    process.exit(code);
});
