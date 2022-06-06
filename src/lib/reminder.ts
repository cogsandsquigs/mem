import { writable } from "svelte/store"

export const destroyIds = writable()
export function destroy(id: number) {
	destroyIds.set(id)
}
