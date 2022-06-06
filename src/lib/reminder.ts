import { writable } from "svelte/store"

// doing this to assert type for reminders
let l: number[] = []
export const reminders = writable(l)

export function add(id: number) {
	reminders.update((ids) => {
		ids.push(id)
		return ids
	})
}

export function remove(id: number) {
	reminders.update((ids) => {
		ids.filter((x: number) => x !== id)
	})
}
