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

export const versionsSchema = z.array(
	z.object({
	  game_versions: z.array(z.string()),
	  loaders: z.array(z.string()),
	  id: z.string(),
	  project_id: z.string(),
	  author_id: z.string(),
	  featured: z.boolean(),
	  name: z.string(),
	  version_number: z.string(),
	  changelog: z.string(),
	  changelog_url: z.null(),
	  date_published: z.string(),
	  downloads: z.number(),
	  version_type: z.string(),
	  status: z.string(),
	  requested_status: z.null(),
	  files: z.array(
		z.object({
		  hashes: z.object({ sha512: z.string(), sha1: z.string() }),
		  url: z.string(),
		  filename: z.string(),
		  primary: z.boolean(),
		  size: z.number(),
		  file_type: z.null()
		})
	  ),
	  dependencies: z.array(
		z.union([
		  z.object({
			version_id: z.string(),
			project_id: z.string(),
			file_name: z.null(),
			dependency_type: z.string()
		  }),
		  z.object({
			version_id: z.null(),
			project_id: z.null(),
			file_name: z.string(),
			dependency_type: z.string()
		  })
		])
	  )
	})
  )

export const projectsSchema = z.array(
	z.object({
		client_side: z.string(),
		server_side: z.string(),
		game_versions: z.array(z.string()),
		id: z.string(),
		slug: z.string(),
		project_type: z.string(),
		team: z.string(),
		organization: z.nullable(z.string()),
		title: z.string(),
		description: z.string(),
		body: z.string(),
		body_url: z.nullable(z.string()),
		published: z.string(),
		updated: z.string(),
		approved: z.nullable(z.string()),
		queued: z.nullable(z.string()),
		status: z.string(),
		requested_status: z.nullable(z.string()),
		moderator_message: z.nullable(z.object({
			message: z.string(),
			body: z.string()
		})),
		license: z.object({ id: z.string(), name: z.string(), url: z.nullable(z.string()) }),
		downloads: z.number(),
		followers: z.number(),
		categories: z.array(z.string()),
		additional_categories: z.array(z.unknown()),
		loaders: z.array(z.string()),
		versions: z.array(z.string()),
		icon_url: z.nullable(z.string()),
		issues_url: z.nullable(z.string()),
		source_url: z.nullable(z.string()),
		wiki_url: z.nullable(z.string()),
		discord_url: z.nullable(z.string()),
		donation_urls: z.array(
			z.object({
				id: z.string(),
				platform: z.string(),
				url: z.string()
			})
		),
		gallery: z.array(
		z.object({
			url: z.string(),
			raw_url: z.string(),
			featured: z.boolean(),
			title: z.nullable(z.string()),
			description: z.nullable(z.string()),
			created: z.string(),
			ordering: z.number()
		})
		),
		color: z.nullable(z.number()),
		thread_id: z.string(),
		monetization_status: z.string()
	}),
)
