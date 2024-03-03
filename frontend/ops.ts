import { readText } from 'https://deno.land/x/dtils@2.4.0/mod.ts'
import { bundle } from 'https://deno.land/x/emit@0.38.2/mod.ts'
import { parseArgs } from 'https://deno.land/std@0.218.2/cli/mod.ts'

const flags = parseArgs(Deno.args, {
	boolean: ['production', 'serve'],
	string: ['title', 'events-url'],
})

const html = await readText(new URL('index.html', import.meta.url).pathname)
const { code } = await bundle(new URL('main.ts', import.meta.url), { minify: flags.production })

const built = html
	.replaceAll('{title}', flags.title || 'Frontend')
	.replaceAll('{script}', code)
	.replaceAll('{events_url}', flags['events-url'] || '')

if (flags.serve) {
	Deno.serve({ port: 3000 }, () => {
		return new Response(built, { headers: { 'Content-Type': 'text/html' } })
	})
}
