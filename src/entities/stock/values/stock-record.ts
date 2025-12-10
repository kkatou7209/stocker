import { Quantity } from '@/entities/stock/values/quantity';
import { SupplyId } from '@/entities/stock/values/supply-id';
import { ArgumentError } from '@/shared/errors';

export class StockRecord {
	private _supplyId: SupplyId;

	private _quantity: Quantity;

	private _recordedAt: Date;

	private constructor(
		supplyId: SupplyId,
		quantity: Quantity,
		recordedAt: Date,
	) {
		this._supplyId = supplyId;
		this._quantity = quantity;
		this._recordedAt = recordedAt;
	}

	public static of = (
		supplyId: SupplyId,
		quantity: Quantity,
		recordedAt: Date,
	): StockRecord => {
		ArgumentError.throwIfAnyNullish(supplyId, quantity, recordedAt);
		ArgumentError.throwIfNotInstanceOf(supplyId, SupplyId);
		ArgumentError.throwIfNotInstanceOf(quantity, Quantity);
		ArgumentError.throwIfNotInstanceOf(recordedAt, Date);
		return new StockRecord(supplyId, quantity, recordedAt);
	};

	public supplyId = () => this._supplyId;

	public quantity = () => this._quantity;

	public recordedAt = () => this._recordedAt;
}
