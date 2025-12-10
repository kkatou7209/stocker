export interface SupplyAccordionValue {
	supplyId: string;
	supplyName: string;
	unitName: string;
}

export interface SuppliesAccordionValue {
	supplierId: string;
	supplierName: string;
	supplies: SupplyAccordionValue[];
}
