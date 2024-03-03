document.addEventListener('DOMContentLoaded', () => {
	const wsPath = document.body.getAttribute('events-url')
	if (wsPath) connect(new URL(wsPath, location.href))
})

function connect(url: URL) {
	applySessionId(url)
	const ws = new WebSocket(url)

	ws.onmessage = ({ data }) => {
		if (typeof data !== 'string') console.error('received unknown message from socket')
		const messages = JSON.parse(data) as Message[]

		for (const message of messages) executeMessage(message)
	}

	ws.onclose = () => {
		console.log('disconnected from server... reconnecting in 3 seconds')
		setTimeout(() => connect(url), 3000)
	}
}

function executeMessage(message: Message) {
	if (message.$ === 'SetPageTitle') {
		document.title = message.title
		return
	}

	if (message.$ === 'SetChildren') {
		const element = document.getElementById(message.id)
		if (!element) return console.error(`received unknown element id: ${message.id}`)

		for (const childIndex in element.children) {
			const child = element.children[childIndex]
			child.remove()
		}

		for (const child of message.children) element.appendChild(createElement(child))

		return
	}

	// TODO

	console.error('unsupported message:', message)
}

function createElement(element: GenericElement) {
	const node = document.createElement(element.tag)
	node.id = element.id
	node.setAttribute('class', element.class_list)

	return node
}

export type Message = SetChildren | SetClassList | AddCssChunk | DeleteCssChunk | SetPageHash | SetPageTitle | ActionTrigger

export interface SetChildren {
	$: 'SetChildren'
	id: string
	children: GenericElement[]
}

export interface SetClassList {
	$: 'SetClassList'
	id: string
	class_list: string
}

export interface AddCssChunk {
	$: 'AddCssChunk'
	id: string
	content: string
}

export interface DeleteCssChunk {
	$: 'DeleteCssChunk'
	id: string
	content: string
}

export interface SetPageHash {
	$: 'SetPageHash'
	hash: string
}

export interface SetPageTitle {
	$: 'SetPageTitle'
	title: string
}

export interface ActionTrigger {
	$: 'ActionTrigger'
	name: string
	data: ElementActionPayload
}

export interface OnHashChanged {
	$: 'OnHashChanged'
	new_hash: string
}

export interface GenericElement {
	tag: string
	id: string
	actions: ActionDefinition[]
	children: GenericElement[]
	class_list: string
	delete_after_ms: number | null
}

export interface ActionDefinition {
	event: string
	show_loader: boolean
	name: string
	data: ElementActionPayloadKind
}

export type ElementActionPayload = ElementActionPayloadInputValue | ElementActionPayloadInputNothing | ElementActionPayloadInputSwitch

export interface ElementActionPayloadInputValue {
	$: 'input_value'
	value: string
}

export interface ElementActionPayloadInputNothing {
	$: 'nothing'
}

export interface ElementActionPayloadInputSwitch {
	$: 'switch'
	value: boolean
}

export type ElementActionPayloadKind = 'input_value' | 'nothing' | 'switch'

function applySessionId(url: URL) {
	if (url.searchParams.has('session_id')) return

	let sessionId = localStorage.getItem('session_id')
	if (!sessionId) {
		sessionId = crypto.randomUUID()
		localStorage.setItem('session_id', sessionId)
	}

	url.searchParams.set('session_id', sessionId)
}
