import { get } from 'svelte/store'

import Room, { Player } from '../models/room'
import { playerStore } from '../store/player'
import { roomStore } from '../store/room_store'
import apiClient from './api_client'

async function createRoom(): Promise<Room> {
	const res = await apiClient.post('/rooms')
	const room = Room.fromApiRoom(res.data)

	// Resetting player if it's another room
	if (get(roomStore).id !== room.id) {
		playerStore.set(null)
	}

	roomStore.set(room)

	return room
}

async function fetchRoom(room_id: string): Promise<Room> {
	const res = await apiClient.get('/games/status', { room_id })
	const room = Room.fromApiRoom({ ...res.data, id: room_id })

	roomStore.set(room)

	return room
}

async function joinRoom(name: string): Promise<Player> {
	const res = await apiClient.post('/players', { room_id: get(roomStore).id, name })
	const player: Player = res.data

	playerStore.set(player)

	return player
}

async function startGame(): Promise<void> {
	await apiClient.post('/games/start', null, { room_id: get(roomStore).id })
}

const api = {
	createRoom: createRoom,
	fetchRoom: fetchRoom,
	joinRoom: joinRoom,
	startGame: startGame,
}

export default api
