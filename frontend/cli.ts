import { writeText } from 'https://deno.land/x/dtils@2.4.0/mod.ts'
import { fromArgs, fromTailwind } from './from_tailwind.ts'
import { DOMParser, Element } from 'https://deno.land/x/deno_dom@v0.1.45/deno-dom-wasm.ts'
import { readText } from 'https://deno.land/x/dtils@2.4.0/fs.ts'

const command = Deno.args[0]

if (command === 'from-html') await fromArgs(Deno.args.slice(1))
if (command === 'gen-example') {
	const tailwindExamples = await fetch('https://tailwindui.com/components/preview').then((res) => res.text())
	const document = new DOMParser().parseFromString(tailwindExamples, 'text/html')
	if (!document) throw new Error('Failed to parse')

	const frames = document.querySelectorAll('iframe')
	const mods: string[] = []
	const widgets: string[] = []

	let index = 0
	for (const node of frames) {
		const frame = node as Element
		const html = frame.getAttribute('srcdoc')
		if (!html) continue

		const document = new DOMParser().parseFromString(html, 'text/html')
		if (!document || !document.body.children.length) continue

		const file = `component_${index}`
		const name = `Example${index}`
		const rust = fromTailwind(document.body.innerHTML, name)

		await writeText(`examples/tailwind/${file}.rs`, rust)
		mods.push(`mod ${file}`)
		widgets.push(`${file}::${name}`)

		index++
	}

	const mainRs = await readText('frontend/tailwind.rs').then((text) =>
		text
			.replace('mods!()', mods.join(';\n'))
			.replace('widget1!()', widgets[1])
	)

	await writeText('examples/tailwind/main.rs', mainRs)
}
