import Room from '../models/room'
import { roomStore } from '../store/room_store'
import apiClient from './api_client'

async function createRoom(): Promise<Room> {
	const res = await apiClient.post('/rooms')
	const room = Room.fromApiRoom(res.data)

	roomStore.set(room)

	return room
}

async function fetchRoom(room_id: string): Promise<Room> {
	const res = await apiClient.get('/games/status', { room_id })
	const room = Room.fromApiRoom({ ...res.data, id: room_id })

	roomStore.set(room)

	return room
}

const api = {
	createRoom: createRoom,
	fetchRoom: fetchRoom,
}

export default api
