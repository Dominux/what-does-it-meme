import { writable } from 'svelte/store'
import Room from '../models/room'

const json: Room = JSON.parse(localStorage.getItem('room'))
const storedRoom = new Room(
	json.id,
	json.state,
	json.players,
	json.round,
	json.expiration_timestamp
)
export const roomStore = writable(storedRoom)
roomStore.subscribe((value) => localStorage.setItem('room', JSON.stringify(value)))
