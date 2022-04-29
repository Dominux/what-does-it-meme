import axios from 'axios'

/**
 * Api client to work with
 */
export class ApiClient {
	constructor(public apiURI: string) {}

	async get(path: string, params: Object) {
		return axios.get(path, { baseURL: this.apiURI, params: params })
	}

	async post(path: string, params: Object, data: Object) {
		return axios.get(path, { baseURL: this.apiURI, params: params, data: data })
	}
}

const apiClient = new ApiClient('localhost:8081')
export default apiClient
