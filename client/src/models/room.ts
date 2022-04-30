export default class Room {
	constructor(
		public id: string,
		public state: RoomState,
		public players: Array<Player>,
		public round: Round,
		public expiration_timestamp: Date
	) {}

	static fromApiRoom(apiRoom: ApiRoom): Room {
		return new Room(
			apiRoom.id,
			apiRoom.state,
			apiRoom.players_names?.map((name) => {
				return { name }
			}),
			apiRoom.round,
			new Date(apiRoom.expiration_timestamp.secs_since_epoch * 1000) // casting seconds to milliseconds
		)
	}
}

interface ApiRoom {
	id: string
	state: RoomState
	players_names: Array<string>
	round: Round
	expiration_timestamp: { secs_since_epoch: number }
}

export enum RoomState {
	NotStarted = 'NotStarted',
	Started = 'Started',
	Ended = 'Ended',
}

export interface Player {
	id?: string
	name: string
	memes_in_hand?: Array<string>
}

type Round = {
	id: string
	round_number: number
	round_state: RoundState
	situation_creator_name: string
	situation: string
	memes: Array<RoundMeme>
	reacted_players_names: Array<string>
}

enum RoundState {
	SituationCreation = 'SITUATION_CREATION',
	ChoosingMemes = 'CHOOSING_MEMES',
	Voting = 'VOTING',
	ShowingResults = 'SHOWING_RESULTS',
	Ended = 'ENDED',
}

type RoundMeme = {
	meme_id: string
	link: string
	author_name: string
	voters_names: Array<string>
}
