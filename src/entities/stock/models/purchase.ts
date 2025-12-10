import { PurchaseAmount } from '@/entities/stock/values/purchase-amount';
import { PurchaseId } from '@/entities/stock/values/purchase-id';
import { SupplyId } from '@/entities/stock/values/supply-id';
import { ArgumentError } from '@/shared/errors';

export class Purchase {
	private readonly _id: PurchaseId;

	private readonly _supplyId: SupplyId;

	private _amount: PurchaseAmount;

	private constructor(
		id: PurchaseId,
		supplyId: SupplyId,
		amount: PurchaseAmount,
	) {
		this._id = id;
		this._supplyId = supplyId;
		this._amount = amount;
	}

	public static restore(
		id: PurchaseId,
		supplyId: SupplyId,
		amount: PurchaseAmount,
	): Purchase {
		ArgumentError.throwIfAnyNullish(id, supplyId, amount);
		ArgumentError.throwIfNotInstanceOf(id, PurchaseId);
		ArgumentError.throwIfNotInstanceOf(supplyId, SupplyId);
		ArgumentError.throwIfNotInstanceOf(amount, PurchaseAmount);
		return new Purchase(id, supplyId, amount);
	}

	public id = () => this._id;

	public supplyId = () => this._supplyId;

	public amount = () => this._amount;

	public changeAmount = (amount: PurchaseAmount) => {
		ArgumentError.throwIfAnyNullish(amount);
		ArgumentError.throwIfNotInstanceOf(amount, PurchaseAmount);
		this._amount = amount;
	};
}
