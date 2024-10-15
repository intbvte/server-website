// place files you want to import through the `$lib` alias in this folder.

import type { z } from 'zod';

export async function fetchWithSchema<T extends z.ZodTypeAny>(
	req: Request,
	schema: T
): Promise<{ data: z.infer<T>; success: true } | { success: false, data: null }> {
	const data = await fetch(req);
	if (!data.ok) return { success: false, data: null };
	const res = await schema.safeParseAsync(await data.json());
	return { success: res.success, data: res.data };
}
