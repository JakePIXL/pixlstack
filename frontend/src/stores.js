import { writable } from 'svelte/store';

export const user = writable();

export const authenticated = writable(false);