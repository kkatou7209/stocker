import { ArgumentError } from '@/shared/errors';

/**
 * quantity of stock
 */
export class Quantity {
	private _value: number;

	public value = (): number => this._value;

	private constructor(value: number) {
		this._value = value;
	}

	public static of = (value: number): Quantity => {
		ArgumentError.throwIfNullish(value);
		ArgumentError.throwIfNotTypeOf(value, 'number');
		return new Quantity(value);
	};

	public sum = (quantity: Quantity): Quantity => {
		ArgumentError.throwIfNullish(quantity);

		const value = this._value + quantity._value;

		return new Quantity(value);
	};

	public equals = (other: Quantity): boolean => {
		ArgumentError.throwIfNotInstanceOf(other, Quantity);
		return this._value === other._value;
	};
}
