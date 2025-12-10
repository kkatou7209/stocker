import { ArgumentError } from '@/shared/errors';

/**
 * name of supply
 */
export class SupplyName {
	private _value: string;

	public value = (): string => this._value;

	private constructor(value: string) {
		this._value = value;
	}

	public static of = (value: string): SupplyName => {
		ArgumentError.throwIfWhitespcae(value);
		ArgumentError.throwIfNotTypeOf(value, 'string');
		return new SupplyName(value);
	};

	public equals = (other: SupplyName): boolean => {
		return this._value === other._value;
	};
}
