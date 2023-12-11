import { Triceratops } from "./index";

export class TriceratopsUsers {
	private readonly client: Triceratops;

	public constructor(client: Triceratops) {
		this.client = client;
	}

	public async get(id: string): Promise<TriceratopsUser | null> {
		try {
			const user = await fetch(`${this.client.url}/api/users/${id}`, {
				method: "GET",
				headers: {
					Authorization: this.client.authentication,
				},
			});

			if (user.status === 200) {
				const userJson = await user.json();

				return new TriceratopsUser(this.client, userJson);
			}

			if (user.status === 404) {
				return null;
			}

			if (user.status === 401) {
				throw new Error("Unauthorized");
			}

			// biome-ignore lint/suspicious/noExplicitAny: Its an error type and im shit at typescript, fuck this shitty language.
		} catch (error: any) {
			throw new Error(`Failed to get user: ${error.toString()}`);
		}

		return null;
	}
}

interface UserObject {
	id: string;
	username: string;
	email: string;
	firstName?: string;
	lastName?: string;
	lastLoginAt?: string;
	createdAt: string;
}

export class TriceratopsUser {
	private client: Triceratops;

	public readonly id: string;

	public readonly username: string;

	public readonly email: string;

	public readonly firstName?: string;

	public readonly lastName?: string;

	public readonly lastLoginAt?: Date;

	public readonly createdAt: Date;

	public constructor(client: Triceratops, user: UserObject) {
		this.client = client;

		this.id = user.id;
		this.username = user.username;
		this.email = user.email;
		this.firstName = user.firstName;
		this.lastName = user.lastName;

		if (user.lastLoginAt) {
			this.lastLoginAt = new Date(user.lastLoginAt);
		}

		this.createdAt = new Date(user.createdAt);
	}
}
