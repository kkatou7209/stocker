import { ArgumentError } from '@/shared/errors';

/**
 * id of supplier
 */
export class SupplierId {
	private _value: string;

	public value = (): string => this._value;

	private constructor(value: string) {
		this._value = value;
	}

	public static of = (value: string): SupplierId => {
		ArgumentError.throwIfWhitespcae(value);
		ArgumentError.throwIfNotTypeOf(value, 'string');
		return new SupplierId(value);
	};

	public equals = (other: SupplierId): boolean => {
		return this._value === other._value;
	};
}
