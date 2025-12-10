import { SupplierId } from '@/entities/stock/values/supplier-id';
import { SupplyId } from '@/entities/stock/values/supply-id';
import { SupplyName } from '@/entities/stock/values/supply-name';
import { UnitName } from '@/entities/stock/values/unit-name';
import { ArgumentError } from '@/shared/errors';

/**
 * supply
 */
export class Supply {
	private _id: SupplyId;

	private _name: SupplyName;

	private _unitName: UnitName;

	private _supplierId: SupplierId;

	public constructor(
		id: SupplyId,
		name: SupplyName,
		unitName: UnitName,
		supplierId: SupplierId,
	) {
		this._id = id;
		this._name = name;
		this._unitName = unitName;
		this._supplierId = supplierId;
	}

	public static restore = (
		id: SupplyId,
		name: SupplyName,
		unitName: UnitName,
		supplierId: SupplierId,
	): Supply => {
		ArgumentError.throwIfAnyNullish(id, name, unitName, supplierId);
		ArgumentError.throwIfNotInstanceOf(id, SupplyId);
		ArgumentError.throwIfNotInstanceOf(name, SupplyName);
		ArgumentError.throwIfNotInstanceOf(unitName, UnitName);
		ArgumentError.throwIfNotInstanceOf(supplierId, SupplierId);
		return new Supply(id, name, unitName, supplierId);
	};

	public id = () => this._id;

	public name = () => this._name;

	public unitName = () => this._unitName;

	public supplierId = () => this._supplierId;

	public rename = (name: SupplyName): void => {
		ArgumentError.throwIfNotInstanceOf(name, SupplyName);
		this._name = name;
	};

	public is = (other: Supply): boolean => {
		ArgumentError.throwIfNotInstanceOf(other, Supply);
		return this._id === other._id;
	};
}
