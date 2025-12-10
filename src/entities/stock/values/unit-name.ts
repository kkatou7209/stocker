import { ArgumentError } from '@/shared/errors';

/**
 * name of unit
 */
export class UnitName {
	private _value: string;

	public value = (): string => this._value;

	private constructor(value: string) {
		this._value = value;
	}

	public static of = (value: string): UnitName => {
		ArgumentError.throwIfWhitespcae(value);
		ArgumentError.throwIfNotTypeOf(value, 'string');
		return new UnitName(value);
	};

	public equals = (other: UnitName): boolean => {
		return this._value === other._value;
	};
}
