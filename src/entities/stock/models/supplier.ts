import z from 'zod';
import { Supply } from '@/entities/stock/models/supply';

export const Supplier = z.object({
	id: z.string().trim().nonempty().readonly(),
	name: z.string().trim().nonempty(),
	supplies: z.array(Supply),
});

/**
 * supplier model
 */
export type Supplier = z.infer<typeof Supplier>;
