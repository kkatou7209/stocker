import { Calendar1Icon, CalendarDaysIcon, CalendarIcon } from 'lucide-solid';
import * as luxon from 'luxon';
import { type Component, createEffect, createSignal } from 'solid-js';

const DateInput: Component<{
	value?: Date | null;
	label?: string;
	onChange?: (value: Date | null) => unknown;
}> = (props) => {

    // biome-ignore lint/style/useConst: ref must be define let
    let dateInput: HTMLInputElement = document.createElement('input');

	const [dateString, setDateString] = createSignal('');

    const click = () => {
        dateInput.showPicker();
    }

	const onChange = (e: { currentTarget: HTMLInputElement }) => {
		const value = e.currentTarget.value;

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
    })

	return (
		<label class="floating-label input max-w-60">
			{props.label ? <span>{props.label}</span> : ''}
			<input type="text" class='caret-transparent' readonly value={dateString()} placeholder={props.label}/>
            <p class='label cursor-pointer w-fit gap-0' onclick={click} >
                <CalendarDaysIcon class='size-4 label'/>
                <input ref={dateInput} type="date" class="h-0 w-0" onchange={onChange} />
            </p>
		</label>
	);
};

export default DateInput;
