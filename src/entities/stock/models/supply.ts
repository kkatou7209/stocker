import z from 'zod';

export const Supply = z.object({
	id: z.string().trim().nonempty().readonly(),
	name: z.string().trim().nonempty(),
	unitName: z.string().trim().nonempty(),
	supplierId: z.string().trim().nonempty().readonly(),
});

/**
 * supply model
 */
export type Supply = z.infer<typeof Supply>;
