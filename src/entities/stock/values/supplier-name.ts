import { ArgumentError } from '@/shared/errors';

/**
 * name of supply
 */
export class SupplierName {
	private _value: string;

	public value = (): string => this._value;

	private constructor(value: string) {
		this._value = value;
	}

	public static of = (value: string): SupplierName => {
		ArgumentError.throwIfWhitespcae(value);
		ArgumentError.throwIfNotTypeOf(value, 'string');
		return new SupplierName(value);
	};

	public equals = (other: SupplierName): boolean => {
		return this._value === other._value;
	};
}
