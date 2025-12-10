import { ArgumentError } from '@/shared/errors';

/**
 * unit price of supply
 */
export class UnitPrice {
	private _value: number;

	public value = (): number => this._value;

	private constructor(value: number) {
		this._value = value;
	}

	public static of = (value: number): UnitPrice => {
		ArgumentError.throwIfNullish(value);
		ArgumentError.throwIfNotTypeOf(value, 'number');
		return new UnitPrice(value);
	};

	public equals = (other: UnitPrice): boolean => {
		return this._value === other._value;
	};
}
