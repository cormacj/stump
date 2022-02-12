enum UserRole {
	Owner = 'owner',
	Member = 'member'
}

// No password field intentionally
interface User {
	/**
	 * The id of the user.
	 */
	id: number;
	/**
	 * The username of the user.
	 */
	username: string;
	/**
	 * The role of the user.
	 */
	role: UserRole;
}

interface UserCredentials {
	username: string;
	password: string;
}
