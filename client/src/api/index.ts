import { get } from 'svelte/store'

import Room, { Player } from '../models/room'
import { playerStore } from '../store/player_store'
import { roomStore } from '../store/room_store'
import { scoreStore } from '../store/score_store'
import apiClient from './api_client'

async function createRoom(): Promise<Room> {
	const res = await apiClient.post('/rooms')
	const room = Room.fromApiRoom(res.data)

	// Resetting player if it's another room
	if (get(roomStore)?.id !== room.id) {
		playerStore.set(null)
	}

	roomStore.set(room)

	return room
}

async function fetchRoom(room_id: string): Promise<Room> {
	const res = await apiClient.get('/games/status', { room_id })
	const room = Room.fromApiRoom({ ...res.data, id: room_id })

	// Resetting player if it's another room
	if (get(roomStore)?.id !== room.id) {
		playerStore.set(null)
	}

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

async function createSituation(situation: string): Promise<void> {
	await apiClient.post('/games/create_situation', {
		round_id: get(roomStore).round.id,
		player_id: get(playerStore).id,
		situation,
	})
}

async function reactWithMeme(link: string) {
	await apiClient.post('/games/react_with_meme', {
		link,
		player_id: get(playerStore).id,
		round_id: get(roomStore).round?.id,
	})
}

async function vote(meme_id: string) {
	await apiClient.post('/games/vote', {
		meme_id,
		player_id: get(playerStore).id,
		round_id: get(roomStore).round?.id,
	})
}

async function getScore(): Promise<{ [key: string]: number }> {
	const res = await apiClient.get('/games/score', { room_id: get(roomStore)?.id })
	scoreStore.set(res.data)

	return res.data
}

const api = {
	createRoom: createRoom,
	fetchRoom: fetchRoom,
	joinRoom: joinRoom,
	startGame: startGame,
	createSituation: createSituation,
	reactWithMeme: reactWithMeme,
	vote: vote,
	getScore: getScore,
}

export default api
