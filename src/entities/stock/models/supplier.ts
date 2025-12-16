import z from 'zod';
import { Supply } from '@/entities/stock/models/supply';

export const Supplier = z.object({
	/**
	 * id of journal
	 */
	id: z.string().trim().nonempty().readonly(),
	/**
	 * name of supplier
	 */
	name: z.string().trim().nonempty(),
	/**
	 * supplies that supplier holds
	 */
	supplies: z.array(Supply),
});

/**
 * supplier model
 */
export type Supplier = z.infer<typeof Supplier>;
