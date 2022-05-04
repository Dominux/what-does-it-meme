import axios from 'axios'

/**
 * Api client to work with
 */
export class ApiClient {
	constructor(readonly apiURI: string) {}

	protected buildAbsolutePath(path: string): string {
		return new URL(path, this.apiURI).toString()
	}

	async get(path: string, params?: Object) {
		return await axios.get(this.buildAbsolutePath(path), { params: params }).catch((error) => {
			throw Error(error.response.data)
		})
	}

	async post(path: string, data?: Object, params?: Object) {
		return await axios
			.post(this.buildAbsolutePath(path), data, { params: params })
			.catch((error) => {
				throw Error(error.response.data)
			})
	}
}

// const apiClient = new ApiClient('http://localhost:10001')
const apiClient = new ApiClient('https://what-does-it-meme.herokuapp.com')
export default apiClient
