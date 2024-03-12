import { writeText } from 'https://deno.land/x/dtils@2.4.0/mod.ts'
import { fromArgs, fromTailwind } from './from_tailwind.ts'
import { DOMParser, Element } from 'https://deno.land/x/deno_dom@v0.1.45/deno-dom-wasm.ts'

const command = Deno.args[0]

if (command === 'from-html') await fromArgs(Deno.args.slice(1))
if (command === 'gen-example') {
	const tailwindExamples = await fetch('https://tailwindui.com/components/preview').then((res) => res.text())
	const document = new DOMParser().parseFromString(tailwindExamples, 'text/html')
	if (!document) throw new Error('Failed to parse')

	const frames = document.querySelectorAll('iframe')
	const mods: string[] = []

	let index = 0
	for (const node of frames) {
		const frame = node as Element
		const html = frame.getAttribute('srcdoc')
		if (!html) continue

		const document = new DOMParser().parseFromString(html, 'text/html')
		if (!document) continue

		const file = `component_${index}`
		const rust = fromTailwind(document.body.innerHTML, `Example${index}`)

		await writeText(`examples/tailwind/${file}.rs`, rust)
		mods.push(`mod ${file};`)

		index++
	}

	const mainRs = `${mods.join('\n')}\n\nfn main() {}\n`
	await writeText('examples/tailwind/main.rs', mainRs)
}
