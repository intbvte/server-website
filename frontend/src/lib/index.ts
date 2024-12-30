// place files you want to import through the `$lib` alias in this folder.

import type { z } from 'zod';

export async function safeFetchWithSchema<T extends z.ZodTypeAny>(
	req: Request | string,
	schema: T,
	fetcher: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response> = fetch
): Promise<{ data: z.infer<T>; success: true } | { success: false; data: null }> {
	const data = await fetcher(req);
	if (!data.ok) return { success: false, data: null };
	const res = await schema.safeParseAsync(await data.json());
	return { success: res.success, data: res.data };
}

export async function fetchWithSchema<T extends z.ZodTypeAny>(
	req: Request | string,
	schema: T,
	fetcher: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response> = fetch
): Promise<z.infer<T>> {
	const data = await fetcher(req);
	return await schema.parseAsync(await data.json());
}
