import { Triceratops } from "./index";

interface LoginResponse {
    user: {
        id: string;
        username: string;
        email: string;
        firstName?: string;
        lastName?: string;
        lastLoginAt?: string;
        createdAt: string;
    },
    session: {
        id: string;
        token: string;
        expiresAt?: string;
        createdAt: string;
    }
}


export class TriceratopsAuth {
	private client: Triceratops;

	public constructor(client: Triceratops) {
		this.client = client;
	}

    public async login(username: string, password: string) {
       const req = await fetch(`${this.client.url}/api/auth/login`, {
            
       })
    }
}
