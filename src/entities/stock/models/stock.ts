import { StockRecord } from '@/entities/stock/values/stock-record';
import { ArgumentError } from '@/shared/errors';
import { Quantity } from '../values/quantity';
import { SupplyId } from '../values/supply-id';

/**
 * supply stock
 */
export class Stock {
	private _supplyId: SupplyId;

	private _quantity: Quantity;

	private _records: StockRecord[] = [];

	public constructor(
		supplyId: SupplyId,
		quantity: Quantity,
		records: StockRecord[],
	) {
		this._supplyId = supplyId;
		this._quantity = quantity;
		this._records = [...records];
	}

	public static of = (
		supplyId: SupplyId,
		quantity: Quantity,
		records: StockRecord[],
	): Stock => {
		ArgumentError.throwIfNotInstanceOf(supplyId, SupplyId);
		ArgumentError.throwIfNotInstanceOf(quantity, Quantity);
		ArgumentError.throwIfNotTypeOf(records, 'array');
		for (const record of records) {
			ArgumentError.throwIfNotInstanceOf(record, StockRecord);
		}
		return new Stock(supplyId, quantity, records);
	};

	public supplyId = () => this._supplyId;

	public quantity = () => this._quantity;

	/**
	 * update stock quantity
	 */
	public updateStock = (stock: StockRecord): void => {
		ArgumentError.throwIfNotInstanceOf(stock, StockRecord);

		if (!this._supplyId.equals(stock.supplyId()))
			throw new ArgumentError('different supply');

		this._records.push(stock);

		const quantity = this._records
			.map((r) => r.quantity())
			.reduce((a, b) => a.sum(b), Quantity.of(0));

		this._quantity = quantity;
	};

	public equals = (other: Stock): boolean => {
		ArgumentError.throwIfNotInstanceOf(other, Stock);

		return this._supplyId.equals(other._supplyId);
	};
}
