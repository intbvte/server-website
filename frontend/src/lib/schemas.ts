import { z } from 'zod';

export const userSchema = z.object({
	discord_id: z.number(),
	minecraft_uuid: z.string().nullable(),
	created_at: z.number(),
	last_updated: z.number(),
	is_admin: z.boolean()
});

export const uuidSchema = z.object({
	id: z.string(),
	name: z.string()
})

export const minecraftUserDataSchema = z.object({
	minecraft_username: z.string(),
	properties: z.array(z.object({
		name: z.string(),
		value: z.string()
	}))
})

export const minecraftProfileSchema = z.object({
	timestamp: z.number(),
	profileId: z.string(),
	profileName: z.string(),
	textures: z.object({
		SKIN: z.optional(z.object({
			url: z.string(),
			metadata: z.optional(z.object({
				model: z.union([
					z.literal("slim"),
					z.literal("slim")
				])
			}))
		}))
	})
})