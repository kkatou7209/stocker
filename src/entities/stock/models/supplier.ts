import { SupplierId } from '@/entities/stock/values/supplier-id';
import { SupplierName } from '@/entities/stock/values/supplier-name';
import { ArgumentError } from '@/shared/errors';

export class Supplier {
	private _id: SupplierId;

	private _name: SupplierName;

	private constructor(id: SupplierId, name: SupplierName) {
		this._id = id;
		this._name = name;
	}

	public static restore(id: SupplierId, name: SupplierName): Supplier {
		ArgumentError.throwIfAnyNullish(id, name);
		ArgumentError.throwIfNotInstanceOf(id, SupplierId);
		ArgumentError.throwIfNotInstanceOf(name, SupplierName);
		return new Supplier(id, name);
	}

	public id = () => this._id;

	public name = () => this._name;
}
