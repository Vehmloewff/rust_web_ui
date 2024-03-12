import { readDir, readText, writeText } from 'https://deno.land/x/dtils@2.4.0/mod.ts'
import { Comment, DOMParser, Element, Text } from 'https://deno.land/x/deno_dom@v0.1.45/deno-dom-wasm.ts'
import { pascalCase, snakeCase } from 'https://deno.land/x/case@2.2.0/mod.ts'
import { basename, extname } from 'https://deno.land/std@0.219.0/path/mod.ts'
import { toText } from 'https://deno.land/std@0.219.0/streams/mod.ts'
import { parseArgs } from 'https://deno.land/std@0.219.0/cli/mod.ts'

export async function fromArgs(args: string[]) {
	const options = parseArgs(args, { string: ['output-file', 'input-file', 'name', 'input-dir', 'output-dir'] })

	const inputDir = options['input-dir']
	if (inputDir) {
		const outputDir = options['output-dir']
		if (!outputDir) throw new Error('Because --input-dir was passed, expected to find --output-dir')

		await fromDirectory(inputDir, outputDir)
		return
	}

	const name = options.name || null
	const inputFile = options['input-file']
	const outputFile = options['output-file'] || null

	if (inputFile) {
		return await fromFile(inputFile, name, outputFile)
	}

	if (!name) throw new Error('Cannot build rust widget from stdin unless the --name flag is passed')

	return await fromStdin(name, outputFile)
}

async function fromFile(path: string, nameOverride: string | null, outputFile: string | null) {
	const html = await readText(path)
	const name = nameOverride ?? basename(path, extname(path))
	const rust = fromTailwind(html, name)

	if (outputFile) await writeText(outputFile, rust)
	else await Deno.stdout.write(new TextEncoder().encode(rust))
}

async function fromStdin(name: string, outputFile: string | null) {
	const html = await toText(Deno.stdin.readable)
	const rust = fromTailwind(html, name)

	if (outputFile) await writeText(outputFile, rust)
	else await Deno.stdout.write(new TextEncoder().encode(rust))
}

async function fromDirectory(inputDir: string, outputDir: string) {
	const files = await readDir(inputDir)

	for (const file of files) {
		console.log(`Parse ${file}`)
		const html = await readText(file)
		const name = basename(file, extname(file))
		const rust = fromTailwind(html, name)

		await writeText(`${outputDir}/${snakeCase(name)}.rs`, rust)
	}
}

export function fromTailwind(html: string, name: string): string {
	const document = new DOMParser().parseFromString(html, 'text/html')
	if (!document) throw new Error(`failed to parse html`)

	let notableChildCount = 0
	let lastNotableChild: Element | null = null

	for (let index = 0; index < document.body.childNodes.length; index++) {
		const child = document.body.children[index]

		if (child instanceof Element) {
			notableChildCount++
			lastNotableChild = child
		}

		if (child instanceof Text && child.textContent.trim().length > 0) notableChildCount++
	}

	const rootNode = notableChildCount === 1 && lastNotableChild !== null ? lastNotableChild : document.body
	const inner = buildNode(rootNode, 'ctx')
	const pascalName = pascalCase(name)

	const rust = `use rust_web_ui::*;

pub struct ${pascalName};

pub struct ${pascalName}Props {}

impl Default for ${pascalName}Props {
	fn default() -> ${pascalName}Props {
		${pascalName}Props { }
	}
}

impl Widget<'_> for ${pascalName} {
	type Props = ${pascalName}Props;

	fn render(mut ctx: Ctx<'_>, props: ${pascalName}Props) {
${indent(indent(inner))}
	}
}
`

	return rust
}

function buildNode(element: Element, handle: string): string {
	const inner: string[] = []

	for (let index = 0; index < element.attributes.length; index++) {
		const attribute = element.attributes.item(index)!
		const name = attribute.name
		const value = attribute.value

		if (name === 'class') {
			inner.push(`${handle}.styles(${buildStyles(value)});`)
		} else {
			inner.push(`${handle}.set_attribute("${name}", ${quote(value)});`)
		}
	}

	inner.push('')

	for (let childIndex = 0; childIndex < element.childNodes.length; childIndex++) {
		const child = element.childNodes[childIndex]

		if (child instanceof Comment) {
			inner.push(child.textContent.split('\n').map((line) => `// ${line}`).join('\n'))

			continue
		}

		if (child instanceof Text) {
			const text = child.textContent.trim()
			if (!text.length) continue

			inner.push(`${handle}.child("${childIndex}", Label).run(|props| props.text(${quote(text)}));`)

			continue
		}

		if (child instanceof Element) {
			const element = child as Element
			const tag = element.tagName === 'BODY' ? 'div' : element.tagName.toLowerCase()

			if (tag === 'svg') {
				inner.push(`${handle}.child("${childIndex}", Icon).run(|mut props| {\n${indent(buildIcon(element, 'props'))}\n});`)
			} else {
				inner.push(`${handle}.child("${childIndex}", Dynamic).run("${tag}", |mut props| {\n${indent(buildNode(element, 'props'))}\n});`)
			}

			continue
		}
	}

	return inner.join('\n').trim()
}

function quote(value: string) {
	return value.includes('"') ? `r#"${value}"#` : `"${value}"`
}

function buildIcon(element: Element, handle: string) {
	const className = element.getAttribute('class') || ''
	const inner: string[] = []
	let style = ''
	let currentHeroiconPrefix = 'Solid'

	for (const token of className.split(/\s+/)) {
		if (token.startsWith('hi-')) {
			const heroIconToken = token.slice(3)

			if (heroIconToken === 'mini') {
				currentHeroiconPrefix = 'Mini'
				continue
			}

			if (heroIconToken === 'outline') {
				currentHeroiconPrefix = 'Outline'
				continue
			}

			if (heroIconToken === 'solid') {
				currentHeroiconPrefix = 'Solid'
				continue
			}

			const iconName = `${currentHeroiconPrefix}${pascalCase(token)}`
			currentHeroiconPrefix = 'Solid'

			inner.push(`${handle}.kind(Heroicon::${iconName});`)
			continue
		}

		style += ` ${token}`
	}

	if (style.trim().length) {
		inner.push(`${handle}.style(${buildStyles(style)});`)
	}

	return inner.join('\n').trim()
}

function indent(text: string) {
	return text.split('\n').map((line) => `\t${line}`).join('\n')
}

function buildStyles(className: string) {
	const styles = className.trim().split(/\s+/).map(convertStyle).join(', ')
	return `&[${styles}]`
}

function convertStyle(node: string) {
	const groups = node.split(':')
	const modifiers = groups.slice(0, -1)
	const body = groups.slice(groups.length - 1)[0]
	const parts = body.split('-')

	let converted = convertStyleParts(parts)

	for (const modifier of modifiers.reverse()) {
		converted = convertModifier(modifier, converted)
	}

	return converted
}

function convertStyleParts(parts: string[]) {
	return first(parts, [
		use(parts, [is('w'), size], ([_, size]) => `Style::Width(${size})`),
		use(parts, [is('h'), size], ([_, size]) => `Style::Width(${size})`),
		use(parts, [is('size'), size], ([_, size]) => `Style::Size(${size})`),

		use(parts, [is('p'), size], ([_, size]) => `Style::Padding(${size})`),
		use(parts, [is('pl'), size], ([_, size]) => `Style::PaddingLeft(${size})`),
		use(parts, [is('pr'), size], ([_, size]) => `Style::PaddingRight(${size})`),
		use(parts, [is('pt'), size], ([_, size]) => `Style::PaddingTop(${size})`),
		use(parts, [is('pb'), size], ([_, size]) => `Style::PaddingBottom(${size})`),
		use(parts, [is('px'), size], ([_, size]) => `Style::PaddingX(${size})`),
		use(parts, [is('py'), size], ([_, size]) => `Style::PaddingY(${size})`),

		use(parts, [is('m'), size], ([_, size]) => `Style::Margin(${size})`),
		use(parts, [is('ml'), size], ([_, size]) => `Style::MarginLeft(${size})`),
		use(parts, [is('mr'), size], ([_, size]) => `Style::MarginRight(${size})`),
		use(parts, [is('mt'), size], ([_, size]) => `Style::MarginTop(${size})`),
		use(parts, [is('mb'), size], ([_, size]) => `Style::MarginBottom(${size})`),
		use(parts, [is('mx'), size], ([_, size]) => `Style::MarginX(${size})`),
		use(parts, [is('my'), size], ([_, size]) => `Style::MarginY(${size})`),

		use(parts, [is('space'), is('x'), size], ([_, __, size]) => `Style::SpaceX(${size})`),
		use(parts, [is('space'), is('y'), size], ([_, __, size]) => `Style::SpaceY(${size})`),

		use(parts, [is('flex')], () => `Style::Flex`),
		use(parts, [is('inline'), is('flex')], () => `Style::InlineFlex`),
		use(parts, [is('justify'), is('between')], () => `Style::JustifyBetween`),
		use(parts, [is('justify'), is('center')], () => `Style::JustifyCenter`),
		use(parts, [is('items'), is('center')], () => `Style::ItemsCenter`),

		use(parts, [is('inline'), is('block')], () => `Style::InlineBlock`),
		use(parts, [is('block')], () => `Style::Block`),

		use(parts, [is('rounded')], () => `Style::Rounded`),

		use(parts, [is('bg'), is('gray'), shade], ([_, __, shade]) => `Style::Color(Color::Fg(${shade}))`),
		use(parts, [is('bg'), is('brand'), size, shade], ([_, __, ___, shade]) => `Style::Color(Color::Primary(${shade}))`),
		use(parts, [is('bg'), is('blue'), shade], ([_, __, shade]) => `Style::Color(Color::Notice(${shade}))`),
		use(parts, [is('bg'), is('orange'), shade], ([_, __, shade]) => `Style::Color(Color::Warn(${shade}))`),
		use(parts, [is('bg'), is('red'), shade], ([_, __, shade]) => `Style::Color(Color::Danger(${shade}))`),
		use(parts, [is('bg'), is('green'), shade], ([_, __, shade]) => `Style::Color(Color::Success(${shade}))`),

		use(parts, [is('text'), is('gray'), shade], ([_, __, shade]) => `Style::TextColor(Color::Fg(${shade}))`),
		use(parts, [is('text'), is('brand'), size, shade], ([_, __, ___, shade]) => `Style::TextColor(Color::Primary(${shade}))`),
		use(parts, [is('text'), is('blue'), shade], ([_, __, shade]) => `Style::TextColor(Color::Notice(${shade}))`),
		use(parts, [is('text'), is('orange'), shade], ([_, __, shade]) => `Style::TextColor(Color::Warn(${shade}))`),
		use(parts, [is('text'), is('red'), shade], ([_, __, shade]) => `Style::TextColor(Color::Danger(${shade}))`),
		use(parts, [is('text'), is('green'), shade], ([_, __, shade]) => `Style::TextColor(Color::Success(${shade}))`),

		use(parts, [is('font'), is('semibold')], () => `Style::FontSemibold`),
		use(parts, [is('font'), is('bold')], () => `Style::FontBold`),
		use(parts, [is('font'), is('light')], () => `Style::FontLight`),
	])
}

function is(value: string) {
	return (part: string) => {
		if (part === value) return part

		return null
	}
}

function size(part: string) {
	if (part === 'full') return `Size::Full`

	const float = parseFloat(part)
	if (isNaN(float)) return null

	return `Size::Exact(${Math.round(float * 4)})`
}

function shade(part: string) {
	const int = parseInt(part)
	if (isNaN(int)) return null

	const ratio = int / 900
	return Math.round(ratio * 100).toString()
}

function use(parts: string[], parsers: ((part: string) => string | null)[], cb: (args: string[]) => string) {
	const args: string[] = []

	if (parts.length !== parsers.length) return null

	for (let index = 0; index < parts.length; index++) {
		const parser = parsers[index]
		if (!parser) return null

		const arg = parser(parts[index])
		if (arg === null) return null

		args.push(arg)
	}

	return cb(args)
}

function first(parts: string[], items: (string | null)[]) {
	for (const item of items) {
		if (item !== null) return item
	}

	return `Style::Noop("${parts.join('-')}")`
}

function convertModifier(name: string, body: string) {
	if (name === 'dark') return body

	if (name === 'hover') return `Style::OnHover(&[${body}])`
	if (name === 'active') return `Style::OnActive(&[${body}])`
	if (name === 'focus') return `Style::OnFocus(&[${body}])`

	if (name === 'sm') return `Style::OnScreen(Screen::Small, &[${body}])`
	if (name === 'md') return `Style::OnScreen(Screen::Medium, &[${body}])`
	if (name === 'lg') return `Style::OnScreen(Screen::Large, &[${body}])`
	if (name === 'xl') return `Style::OnScreen(Screen::ExtraLarge(1), &[${body}])`
	if (/^(\d)xl$/.test(name)) return `Style::Screen(Screen::ExtraLarge(${name.slice(0, 1)}), &[${body}])`

	return `Style::NoopGroup("${name}", &[${body}])`
}
