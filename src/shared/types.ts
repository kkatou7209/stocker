import type { CalendarDateProps, CalendarMonthProps } from 'cally';

/**
 * primitive types
 */
export type TypeOf = 'number' | 'string' | 'object' | 'function' | 'array';

export type MapEvents<T> = {
	[K in keyof T as K extends `on${infer E}` ? `on${Lowercase<E>}` : K]: T[K];
};

declare module 'solid-js' {
	namespace JSX {
		interface IntrinsicElements {
			'calendar-date': MapEvents<CalendarDateProps> &
				JSX.HTMLAttributes<HTMLElement>;
			'calendar-month': MapEvents<CalendarMonthProps> &
				JSX.HTMLAttributes<HTMLElement>;
		}
	}
}
