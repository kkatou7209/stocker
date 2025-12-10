import { ArgumentError } from '@/shared/errors';

/**
 * id of suppply
 */
export class SupplyId {
	private _value: string;

	public value = (): string => this._value;

	private constructor(value: string) {
		this._value = value;
	}

	public static of = (value: string): SupplyId => {
		ArgumentError.throwIfWhitespcae(value);
		ArgumentError.throwIfNotTypeOf(value, 'string');
		return new SupplyId(value);
	};

	public equals = (other: SupplyId): boolean => {
		return this._value === other._value;
	};
}
