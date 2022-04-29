type Room = {
	id: string
	state: RoomState
	players: Array<Player>
	round: Round
	expiration_timestamp: Date
}

enum RoomState {
	NotStarted = 'NOT_STARTED',
	Started = 'STARTED',
	Ended = 'ENDED',
}

interface Player {
	id?: string
	name: string
	room_id: string
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
