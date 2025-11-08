export interface SessionUser {
	id: string;
	login: string;
	name?: string | null;
	avatar_url?: string | null;
	email?: string | null;
	role: string;
}

export interface SessionPayload {
	user: SessionUser;
	csrfToken: string;
}
