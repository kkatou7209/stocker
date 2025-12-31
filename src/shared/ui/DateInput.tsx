import 'cally';
import { CalendarDaysIcon, ChevronLeftIcon, ChevronRightIcon } from 'lucide-solid';
import * as luxon from 'luxon';
import { type Component, createEffect, createSignal } from 'solid-js';

const DateInput: Component<{
	value?: Date | null;
	label?: string;
	onChange?: (value: Date | null) => unknown;
}> = (props) => {

	const [dateString, setDateString] = createSignal('');

	let calender: HTMLElement & { value?: string; } | undefined;

	const onChange = () => {

		if (!calender) return;

		const value = calender.value ?? '';

		const date = luxon.DateTime.fromFormat(value, 'yyyy-MM-dd');

		if (!date.isValid) {
			setDateString('');
			props.onChange?.(null);
			return;
		}

		props.onChange?.(date.toJSDate());

		setDateString(date.toFormat('yyyy年M月d日'));
	};

    createEffect(() => {
        if (!props.value) return;

        const date = luxon.DateTime.fromISO(props.value.toISOString());

        setDateString(date.toFormat('yyyy年M月d日'));
    });

	return (
		<div class="floating-label input max-w-60 z-10">
			{props.label ? <span>{props.label}</span> : ''}
			<input type="text" class='caret-transparent' readonly value={dateString()} placeholder={props.label}/>
			<div class='label'>
				<div class='dropdown'>
					<button type='button' tabIndex={0}>
						<CalendarDaysIcon class='size-4 label'/>
					</button>
					<div tabIndex={-1} class='menu dropdown-content bg-base-200 rounded-box shadow-lg'>
						<calendar-date ref={calender} class='cally' onchange={onChange} locale='ja-JP'>
							<ChevronLeftIcon slot='previous' aria-label="Previous"/>
							<ChevronRightIcon slot='next' aria-label="Next"/>
							<calendar-month></calendar-month>
						</calendar-date>
					</div>
				</div>
			</div>
		</div>
	);
};

export default DateInput;
