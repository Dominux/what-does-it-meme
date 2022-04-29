import { writable } from 'svelte/store'
import type Room from '../models/room'

const storedRoom: Room = JSON.parse(localStorage.getItem('room'))
export const roomStore = writable(storedRoom)
roomStore.subscribe((value) => localStorage.setItem('room', JSON.stringify(value)))
