import { ArgumentError } from '@/shared/errors';

/**
 * amount of purchase
 */
export class PurchaseAmount {
	private _value: number;

	public value = (): number => this._value;

	private constructor(value: number) {
		this._value = value;
	}

	public static of = (value: number): PurchaseAmount => {
		ArgumentError.throwIfNullish(value);
		ArgumentError.throwIfNotTypeOf(value, 'number');
		return new PurchaseAmount(value);
	};

	public equals = (other: PurchaseAmount): boolean => {
		return this._value === other._value;
	};
}
