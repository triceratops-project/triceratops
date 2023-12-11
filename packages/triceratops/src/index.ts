import { TriceratopsUsers } from "./users";

interface TriceratopsOptions {
	secure: boolean;
}

export class Triceratops {
	public readonly url: string;
	public readonly authentication: string;

	//  re-exports (Typescript is shit)
	public readonly users: TriceratopsUsers;

	constructor(url: string, token: string, options?: TriceratopsOptions) {
		let schema = "https";
		if (options?.secure === false) {
			schema = "http";
		}

		this.authentication = `Bearer ${token}`;

		this.url = url;

		if (!url.startsWith("http")) {
			this.url = `${schema}://${url}`;
		}

		this.users = new TriceratopsUsers(this);
	}
}
