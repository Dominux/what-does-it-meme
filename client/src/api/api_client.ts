import axios from 'axios'

/**
 * Api client to work with
 */
export class ApiClient {
	constructor(public apiURI: string) {}

	protected buildAbsolutePath(path: string): string {
		return new URL(path, this.apiURI).toString()
	}

	async get(path: string, params?: Object) {
		return await axios.get(this.buildAbsolutePath(path), { params: params }).catch((error) => {
			throw Error(error.response.data)
		})
	}

	async post(path: string, params?: Object, data?: Object) {
		return await axios
			.post(this.buildAbsolutePath(path), { params: params, data: data })
			.catch((error) => {
				throw Error(error.response.data)
			})
	}
}

const apiClient = new ApiClient('http://localhost:10001')
export default apiClient
