import { z } from 'zod';

export const userSchema = z.object({
	discord_id: z.number(),
	discord_username: z.string(),
	minecraft_uuid: z.string().nullable(),
	created_at: z.number(),
	last_updated: z.number(),
	is_admin: z.boolean()
});
