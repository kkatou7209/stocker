import type { TypeOf } from '@/shared/types';

export class ArgumentError extends Error {
	/**
	 * throws {@link ArgumentError} on null or undefined.
	 */
	// biome-ignore lint/suspicious/noExplicitAny: accepts any types
	public static throwIfNullish = (value: any): void => {
		if (value === null || value === undefined)
			throw new ArgumentError(`value was not provided.`);
	};

	/**
	 * throws {@link ArgumentError} if any value provided is null or undefined.
	 */
	// biome-ignore lint/suspicious/noExplicitAny: accepts any types
	public static throwIfAnyNullish = (...values: any[]): void => {
		ArgumentError.throwIfNullish(values);
		for (const value of values) {
			ArgumentError.throwIfNullish(value);
		}
	};

	/**
	 * throws {@link ArgumentError} on whitespace.
	 */
	public static throwIfWhitespcae = (value: string): void => {
		this.throwIfNullish(value);
		if (value.trim() === '')
			throw new ArgumentError('value was only whitespaces');
	};

	/**
	 * throws {@link ArgumentError} on negative number.
	 */
	public static throwIfNegative = (value: number): void => {
		this.throwIfNullish(value);
		if (value < 0) throw new ArgumentError('value was negative');
	};

	public static throwIfNotInstanceOf = (
		// biome-ignore lint/suspicious/noExplicitAny: accepts any type
		value: any,
		// biome-ignore lint/suspicious/noExplicitAny: accepts any type
		constructorType: any,
	): void => {
		if (!(value instanceof constructorType))
			throw new ArgumentError(
				`not a instance of ${constructorType.name}`,
			);
	};

	public static throwIfNotTypeOf = (value: any, type: TypeOf): void => {
		if (typeof value !== type)
			throw new ArgumentError(`not type of ${type}`);
	};
}
