import { ArgumentError } from '@/shared/errors';

/**
 * id of supplier
 */
export class PurchaseId {
	private _value: string;

	public value = (): string => this._value;

	private constructor(value: string) {
		this._value = value;
	}

	public static of = (value: string): PurchaseId => {
		ArgumentError.throwIfWhitespcae(value);
		ArgumentError.throwIfNotTypeOf(value, 'string');
		return new PurchaseId(value);
	};

	public equals = (other: PurchaseId): boolean => {
		return this._value === other._value;
	};
}
