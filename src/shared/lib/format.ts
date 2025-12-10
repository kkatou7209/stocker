export type FormatterLocale = 'ja-JP';

export type CurrencyLoclae = 'JPY';

export interface Formatter<T> {
	readonly format: (value: T) => string;
}

export interface Formatters {
	readonly number: Formatter<number>;
	readonly currency: Formatter<number>;
	readonly date: Formatter<Date>;
}

const getCurrencyLocale = (locale: FormatterLocale): CurrencyLoclae => {
	switch (locale) {
		case 'ja-JP':
			return 'JPY';
		default:
			return 'JPY';
	}
};

export const useFormat = (locale: FormatterLocale): Formatters => {
	const number = Intl.NumberFormat(locale);

	const currency = Intl.NumberFormat(locale, {
		style: 'currency',
		currency: getCurrencyLocale(locale),
	});

	const date = Intl.DateTimeFormat(locale);

	return {
		number,
		currency,
		date,
	};
};
