// @ts-check

const path = require('path')
const cp = require('child_process')
const { promises: fs } = require('fs')

const cwd = path.join(__dirname)
const main = async () => {
	const names = await fs.readdir(path.join(__dirname, 'tools/in'))
	await fs.rm(path.join(__dirname, 'tmpsvgs'), { recursive: true, force: true })
		.catch(Boolean)
	await fs.mkdir(path.join(__dirname, 'tmpsvgs'))
	/** @type {{ name: string, meta: string }[]} */
	const svgnames = []
	for (const name of names) {
		const fpath = path.join(__dirname, 'tools/in', name)
		await fs.copyFile(fpath, path.join(__dirname, 'tmpin.txt'))
		console.log(name)
		cp.execSync('cat tmpin.txt | cargo run --bin a >| tmpout.txt', { cwd })
		const r = cp.execSync('(cd tools && cargo run --release --bin vis ../tmpin.txt ../tmpout.txt)', { cwd })
		if (!Number(r)) return
		await fs.copyFile(path.join(__dirname, 'tools/out.svg'), path.join(__dirname, 'tmpsvgs', name + '.svg'))
		const txt = await fs.readFile(path.join(__dirname, 'tmpin.txt'), 'utf8')
		const linecount = txt.split('\n')[0].trim()
		svgnames.push({
			name: name + '.svg',
			meta: `${name}: ${linecount}`
		})
	}
	await fs.writeFile(path.join(__dirname, 'tmpsvgs', 'index.html'), html(svgnames))
}

const html = (/** @type {{ name: string, meta: string }[]} */ names) =>
	`<html>\n${names
		.map(n => `<p>${n.meta}<br><img src="${n.name}" width="1000" height="1000"></p>\n`)
		.join('')}</html>`

main().catch(x => {
	console.error(x)
	process.exit(1)
})