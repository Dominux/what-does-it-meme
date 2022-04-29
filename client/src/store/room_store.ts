import { writable } from 'svelte/store'

const storedRoom: Room = JSON.parse(localStorage.getItem('room'))
export const room = writable(storedRoom)
room.subscribe((value) => localStorage.setItem('room', JSON.stringify(value)))
